package scsp

import "google.golang.org/grpc"

func NewClientOrDie(target string) SCSPServiceClient {
	conn, err := grpc.Dial(target)
	if err != nil {
		panic(err)
	}
	return NewSCSPServiceClient(conn)
}
