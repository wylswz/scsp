package scsp

import (
	"sync"

	"github.com/wylswz/SCSP/cluster"
	"k8s.io/klog/v2"
)

var (
	connLock          = sync.Mutex{}
	globalLock        = sync.Mutex{}
	connectionManager *ConnectionManager
)

// Poll
// Broadcast
type ConnectionManager struct {
	ss             map[string]SCSPService_RegisterServer
	clusterManager *cluster.ClusterManager
}

func (cm *ConnectionManager) establish(addr string, ss SCSPService_RegisterServer) error {
	connLock.Lock()
	defer connLock.Unlock()
	cm.ss[addr] = ss
	return nil
}

func (cm *ConnectionManager) Broadcast(payload []byte) {
	connLock.Lock()
	defer connLock.Unlock()
	for _, c := range cm.ss {
		err := c.Send(&RegisterResp{
			Payload: payload,
		})
		if err != nil {
			klog.Error(err)
		}
	}
}

func GetConnectionManager() *ConnectionManager {
	globalLock.Lock()
	defer globalLock.Unlock()
	if connectionManager == nil {
		connectionManager = &ConnectionManager{
			clusterManager: cluster.GetClusterManager(),
		}
	}
	return connectionManager
}
