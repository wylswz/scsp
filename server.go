package scsp

import (
	context "context"
	"net"

	"google.golang.org/grpc"
	klog "k8s.io/klog/v2"
)

type SCSPServerImpl struct {
	connectionManager *ConnectionManager
	UnimplementedSCSPServiceServer
}

// Register a slave to a master
// The address is the ip address of client
func (s SCSPServerImpl) Register(m *RegisterMessage, ss SCSPService_RegisterServer) error {
	s.connectionManager.establish(m.Address, ss)
	return nil
}

// Report local clipboard to master
func (s SCSPServerImpl) Report(context.Context, *ClipBoardMessage) (*ClipBoardResp, error) {
	return &ClipBoardResp{}, nil
}

func NewServerOrDie() SCSPServiceServer {
	return SCSPServerImpl{
		connectionManager: GetConnectionManager(),
	}
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
