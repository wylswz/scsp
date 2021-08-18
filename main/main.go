package main

import (
	"flag"
	"os"
	"os/signal"
	"syscall"

	scsp "github.com/wylswz/SCSP"
	"k8s.io/klog/v2"
)

func main() {
	var listen string
	var master string
	var advAddress string
	fs := flag.NewFlagSet("scsp", flag.ExitOnError)
	fs.StringVar(&listen, "listen", "", "Address to listen")
	fs.StringVar(&master, "master", "", "Master address to connect to")
	fs.StringVar(&advAddress, "advAddress", "", "Slave's advertise address")
	err := fs.Parse(os.Args[1:])
	if err != nil {
		klog.Error(err)
		fs.Usage()
	}
	if listen != "" && master != "" {
		fs.Usage()
		panic("Cannot start master and slave at the same time")
	}
	if listen == "" && master == "" {
		fs.Usage()
		panic("Must specify at lease one of listen and master")
	}
	sigc := make(chan os.Signal, 2)
	signal.Notify(sigc, syscall.SIGTERM, syscall.SIGINT)

	var s *scsp.SCSPServerImpl
	var slave *scsp.Slave

	if listen != "" {
		// Start server
		s = scsp.NewServerOrDie()
		go s.Serve(listen)
	} else {
		slave = scsp.NewSlave(master, advAddress)
		go slave.Start()
	}
	for {
		<-sigc
		break
	}
}
