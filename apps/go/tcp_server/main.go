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
	listener, _err := net.Listen("tcp", address)
	if _err != nil {
		return _err
	}

	ns.listener = listener
	log.Info().Msgf("listening TCP on port: ", ns.listener.Addr())

	// connectionIndex, _err := ns.AcceptConnection()
	// if _err != nil {
	// 	return 0, er
	// }

	return nil
}

func (ns *NetworkServer) AcceptConnection() (int, error) {
	c, _err := ns.listener.Accept()
	if _err != nil {
		return 0, _err
	}
	index := len(ns.connection)
	ns.connection = append(ns.connection, c)
	log.Info().Msgf("new connection from:", ns.connection[index].RemoteAddr().String())
	return index, nil
}

func (ns *NetworkServer) Read(index int) ([]byte, error) {
	buff := make([]byte, 1024)
	_, _err := ns.connection[index].Read(buff)
	if _err != nil {
		return nil, _err
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
	index, _err := netServer.AcceptConnection()
	if _err != nil {
		log.Err(_err)
	}
	defer netServer.Close(index)

	data, _err := netServer.Read(index)
	if _err != nil {
		if _err.Error() == "EOF" {
			log.Error().Msg("connection closed by remote host")
			return
		}
		panic(_err.Error())
	}
	log.Info().Msg(string(data))
	fmt.Println(string(data))
	if string(data) == "stop\r\n" {
		return
	}
	log.Info().Msg("stopping server")
}
