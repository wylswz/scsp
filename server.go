package scsp

import (
	context "context"
	"net"

	"google.golang.org/grpc"
	klog "k8s.io/klog/v2"
)

type SCSPServerImpl struct {
	UnimplementedSCSPServiceServer
}

// Register a slave to a master
// The address is the ip address of client
func (s SCSPServerImpl) Register(context.Context, *RegisterMessage) (*RegisterResp, error) {
	return &RegisterResp{}, nil
}

// Report local clipboard to master
func (s SCSPServerImpl) Report(context.Context, *ClipBoardMessage) (*ClipBoardResp, error) {
	return &ClipBoardResp{}, nil
}

// Ping master for liveness check
// If master doesn't reveive pings from slave for TTL,
// it automatically remove slave from registry
func (s SCSPServerImpl) Ping(context.Context, *PingMessage) (*PingResp, error) {
	return &PingResp{}, nil
}

func NewServerOrDie() SCSPServiceServer {
	return SCSPServerImpl{}
}

func ServeOrDie(addr string) {
	s := grpc.NewServer()
	listener, err := net.Listen("ipv4", addr)
	if err != nil {
		panic(err)
	}
	klog.Info("Registering services")
	RegisterSCSPServiceServer(s, NewServerOrDie())

	klog.Info("Server started at ", addr)
	s.Serve(listener)
}
