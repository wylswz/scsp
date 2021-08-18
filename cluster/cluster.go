package cluster

import (
	"sync"
	"time"
)

const (
	// 180s
	TTL = 180
)

var (
	mutex          = sync.Mutex{}
	slaveLock      = sync.Mutex{}
	clusterManager *ClusterManager
)

type Slave struct {
	Addr       string
	TTL        int64
	LastActive int64
}

func NewSlave(addr string) Slave {
	return Slave{
		Addr:       addr,
		TTL:        TTL,
		LastActive: time.Now().Unix(),
	}
}

func (s *Slave) isValid() bool {
	return s.LastActive+s.TTL > time.Now().Unix()
}

func (s *Slave) refresh() {
	s.LastActive = time.Now().Unix()
}

type ClusterManager struct {
	// Add a slave to cluster
	slaves map[string]Slave
	active bool
}

func (cm *ClusterManager) AddSlave(addr string) {
	s, ok := cm.slaves[addr]
	if !ok || !s.isValid() {
		cm.slaves[addr] = NewSlave(addr)
		return
	}
	s.refresh()
}

// Refresh tries to refresh
func (cm *ClusterManager) Refresh(addr string, ts int64) {
	s, ok := cm.slaves[addr]
	if ok {
		s.refresh()
	}
}

func (cm *ClusterManager) Start() {
	mutex.Lock()
	cm.active = true
	mutex.Unlock()

	for cm.active {
		slaveLock.Lock()
		outdated := []string{}
		for k, v := range cm.slaves {
			if !v.isValid() {
				outdated = append(outdated, k)
			}
		}
		for _, o := range outdated {
			delete(cm.slaves, o)
		}
		slaveLock.Unlock()
		time.Sleep(1 * time.Second)
	}
}

func (cm *ClusterManager) ForEach(f func(Slave)) {
	slaveLock.Lock()
	for _, s := range cm.slaves {
		f(s)
	}
}

func GetClusterManager() *ClusterManager {
	if clusterManager == nil {
		mutex.Lock()
		if clusterManager == nil {
			clusterManager = &ClusterManager{
				slaves: map[string]Slave{},
			}
		}
		mutex.Unlock()
	}
	return clusterManager
}
