package main

import (
	"context"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/reflection"
	"google.golang.org/grpc/status"

	"stelo.dev/grpc/calculator/pb"
)

type server struct {
	pb.UnimplementedCalculatorServer
}

func (s *server) Add(
	ctx context.Context,
	req *pb.CalculationRequest,
) (*pb.CalculationResult, error) {
	return &pb.CalculationResult{Result: req.A + req.B}, nil
}

func (s *server) Subtract(
	ctx context.Context,
	req *pb.CalculationRequest,
) (*pb.CalculationResult, error) {
	return &pb.CalculationResult{Result: req.A - req.B}, nil
}

func (s *server) Multiply(
	ctx context.Context,
	req *pb.CalculationRequest,
) (*pb.CalculationResult, error) {
	return &pb.CalculationResult{Result: req.A * req.B}, nil
}

func (s *server) Divide(
	ctx context.Context,
	req *pb.CalculationRequest,
) (*pb.CalculationResult, error) {
	if req.B == 0 {
		return nil, status.Errorf(codes.InvalidArgument, "Cannot divide by zero")
	}

	return &pb.CalculationResult{Result: req.A / req.B}, nil
}

func (s *server) Sum(
	ctx context.Context,
	req *pb.NumbersRequest,
) (*pb.CalculationResult, error) {
	var result int64
	for _, num := range req.Numbers {
		result += num
	}
	return &pb.CalculationResult{Result: result}, nil
}

func main() {
	listener, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Fatalln("Failed to create listener:", err)
	}

	s := grpc.NewServer()
	reflection.Register(s)
	pb.RegisterCalculatorServer(s, &server{})

	if err := s.Serve(listener); err != nil {
		log.Fatalln("Failed to serve:", err)
	}
}
