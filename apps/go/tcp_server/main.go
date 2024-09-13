package main

import (
	"fmt"
	"net"

	"github.com/rs/zerolog/log"
)

type NetworkServer struct {
	listener   net.Listener
	connection []net.Conn
}

func NewTcpServer() *NetworkServer {
	return &NetworkServer{}
}

func (ns *NetworkServer) ListenTcp(address string) error {
	listener, err := net.Listen("tcp", address)
	if err != nil {
		return err
	}

	ns.listener = listener
	log.Info().Msgf("listening TCP on port: ", ns.listener.Addr())

	// connectionIndex, err := ns.AcceptConnection()
	// if err != nil {
	// 	return 0, er
	// }

	return nil
}

func (ns *NetworkServer) AcceptConnection() (int, error) {
	c, err := ns.listener.Accept()
	if err != nil {
		return 0, err
	}
	index := len(ns.connection)
	ns.connection = append(ns.connection, c)
	log.Info().Msgf("new connection from:", ns.connection[index].RemoteAddr().String())
	return index, nil
}

func (ns *NetworkServer) Read(index int) ([]byte, error) {
	buff := make([]byte, 1024)
	_, err := ns.connection[index].Read(buff)
	if err != nil {
		return nil, err
	}
	log.Info().Msg(fmt.Sprintf("recieved message form:", ns.connection[index].RemoteAddr()))
	return ns.removePadding(&buff), nil
}

func (ns *NetworkServer) removePadding(data *[]byte) []byte {
	var trimmedBuff []byte
	for _, a := range *data {
		if string(a) != "\u0000" {
			trimmedBuff = append(trimmedBuff, a)
		}
	}
	return trimmedBuff
}

func (ns *NetworkServer) Close(connectionIndex int) error {
	return ns.connection[connectionIndex].Close()
}

func main() {
	fmt.Println("Hello World!")

	netServer := NewTcpServer()
	netServer.ListenTcp("127.0.0.1:8080")
	index, err := netServer.AcceptConnection()
	if err != nil {
		log.Err(err)
	}
	defer netServer.Close(index)

	data, err := netServer.Read(index)
	if err != nil {
		if err.Error() == "EOF" {
			log.Error().Msg("connection closed by remote host")
			return
		}
		panic(err.Error())
	}
	log.Info().Msg(string(data))
	fmt.Println(string(data))
	if string(data) == "stop\r\n" {
		return
	}
	log.Info().Msg("stopping server")
}
