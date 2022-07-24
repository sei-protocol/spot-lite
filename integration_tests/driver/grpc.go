package driver

import "google.golang.org/grpc"

func getGRPCConn() *grpc.ClientConn {
	grpcConn, _ := grpc.Dial(
		"127.0.0.1:9090",
		grpc.WithInsecure(),
	)
	return grpcConn
}
