package scsp

import (
	"net"

	"google.golang.org/grpc"
	klog "k8s.io/klog/v2"
)

func ServeOrDie(addr string) {
	s := grpc.NewServer()
	listener, err := net.Listen("ipv4", addr)
	if err != nil {
		panic(err)
	}
	klog.Info("Server started at ", addr)
	s.Serve(listener)
}
