package scsp

import (
	"context"
	"sync"

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
		resp, err := s.ClientStream.Recv()
		if err != nil {
			klog.Error(err)
		}
		s.clipBoard.Write(resp.Payload)
	}
}

func (s *Slave) send() {
	s.wg.Add(1)
	defer s.wg.Done()
	for {
		select {
		case content := <-s.clipBoard.Chan():
			s.client.Report(context.TODO(), &ClipBoardMessage{
				Content: content,
			})
		case <-s.close:
			return
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
