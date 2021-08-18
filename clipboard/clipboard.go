package clipboard

import "sync"

var (
	provider ClipBoardProvider
	lock     = sync.Mutex{}
)

type ClipBoardProvider interface {
	Write([]byte)
	Chan() chan []byte
}

type XWindowClipBoardProvider struct {
	c chan []byte
}

func (*XWindowClipBoardProvider) Write([]byte) {

}

func (x *XWindowClipBoardProvider) Chan() chan []byte {
	return x.c
}

func GetProvider() ClipBoardProvider {
	lock.Lock()
	defer lock.Unlock()
	if provider == nil {
		provider = &XWindowClipBoardProvider{
			c: make(chan []byte),
		}
	}
	return provider
}
