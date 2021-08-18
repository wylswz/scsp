package scsp

import (
	context "context"
	"net"

	"google.golang.org/grpc"
)

type SCSPServerImpl struct {
	connectionManager *ConnectionManager
	server            *grpc.Server
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

func (s SCSPServerImpl) Ping(ctx context.Context, m *PingMessage) (*PingResp, error) {
	s.connectionManager.Refresh(m.Address)
	return &PingResp{}, nil
}

func (s *SCSPServerImpl) Serve(addr string) {
	svc := grpc.NewServer()
	listener, err := net.Listen("tcp4", addr)
	if err != nil {
		panic(err)
	}
	RegisterSCSPServiceServer(svc, s)
	s.server = svc
	svc.Serve(listener)
}

func (s SCSPServerImpl) Stop() {
	s.server.Stop()
}

func NewServerOrDie() *SCSPServerImpl {
	return &SCSPServerImpl{
		connectionManager: GetConnectionManager(),
	}
}
