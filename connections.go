package scsp

import (
	"sync"
	"time"

	"k8s.io/klog/v2"
)

const (
	TTL = 180
)

var (
	connLock          = sync.Mutex{}
	globalLock        = sync.Mutex{}
	connectionManager *ConnectionManager
)

type Connection struct {
	ss         SCSPService_RegisterServer
	ttl        int64
	lastActive int64
}

func (c *Connection) refresh() {
	c.lastActive = time.Now().Unix()
}

func (c *Connection) isValid() bool {
	return c.lastActive+c.ttl > time.Now().Unix()
}

// Poll
// Broadcast
type ConnectionManager struct {
	conn map[string]Connection
}

func (cm *ConnectionManager) establish(addr string, ss SCSPService_RegisterServer) error {
	connLock.Lock()
	defer connLock.Unlock()
	cm.conn[addr] = Connection{
		ss:         ss,
		ttl:        180,
		lastActive: time.Now().Unix(),
	}
	return nil
}

func (cm *ConnectionManager) Broadcast(payload []byte) {
	connLock.Lock()
	defer connLock.Unlock()
	for k, c := range cm.conn {
		err := c.ss.Send(&RegisterResp{
			Payload: payload,
		})
		delete(cm.conn, k)
		if err != nil {
			klog.Error(err)
		}
	}
}

func (cm *ConnectionManager) Refresh(addr string) {
	conn, ok := cm.conn[addr]
	if ok {
		conn.refresh()
	}
}

func (cm *ConnectionManager) Sync() {
	connLock.Lock()
	defer connLock.Unlock()
	for k, con := range cm.conn {
		if !con.isValid() {
			delete(cm.conn, k)
		}
	}
}

func GetConnectionManager() *ConnectionManager {
	globalLock.Lock()
	defer globalLock.Unlock()
	if connectionManager == nil {
		connectionManager = &ConnectionManager{
			conn: map[string]Connection{},
		}
	}
	return connectionManager
}
