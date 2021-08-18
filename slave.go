package scsp

import (
	"context"
	"sync"
	"time"

	"github.com/wylswz/SCSP/clipboard"
	"k8s.io/klog/v2"
)

type Slave struct {
	advAddress   string
	client       SCSPServiceClient
	active       bool
	clipBoard    clipboard.ClipBoardProvider
	ClientStream SCSPService_RegisterClient
	close        chan bool
	wg           sync.WaitGroup
}

var (
	lock = sync.Mutex{}
)

func NewSlave(target, advAddress string) *Slave {
	return &Slave{
		advAddress: advAddress,
		client:     NewClientOrDie(target),
		close:      make(chan bool),
		active:     true,
		wg:         sync.WaitGroup{},
		clipBoard:  clipboard.GetProvider(),
	}
}

func (s *Slave) recv() {
	s.wg.Add(1)
	defer s.wg.Done()
	for {
		lock.Lock()
		if !s.active {
			break
		}
		lock.Unlock()
		klog.Info("Waiting for messages")
		resp, err := s.ClientStream.Recv()
		if err != nil {
			klog.Error(err)
		}
		klog.Info("Recved: ", string(resp.Payload))
		s.clipBoard.Write(resp.Payload)
	}
}

func (s *Slave) send() {
	s.wg.Add(1)
	defer s.wg.Done()
	for {
		select {
		case content := <-s.clipBoard.Chan():
			klog.Info("Try to sync msg", string(content))
			s.client.Report(context.Background(), &ClipBoardMessage{
				Content: content,
				Address: s.advAddress,
			})
		case <-s.close:
			return
		default:
			time.Sleep(3 * time.Second)
		}
	}
}

func (s *Slave) Start() error {
	c, err := s.client.Register(context.TODO(), &RegisterMessage{
		Address: s.advAddress,
	})
	if err != nil {
		return err
	}
	s.ClientStream = c
	go s.recv()
	go s.send()
	s.wg.Wait()
	return nil
}

func (s *Slave) Stop() {
	lock.Lock()
	defer lock.Unlock()
	s.active = false
	s.close <- true
}
