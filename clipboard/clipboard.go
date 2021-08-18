package clipboard

import (
	"context"
	"sync"

	"golang.design/x/clipboard"
)

var (
	provider ClipBoardProvider
	lock     = sync.Mutex{}
)

type ClipBoardProvider interface {
	Write([]byte)
	Chan() <-chan []byte
}

type XWindowClipBoardProvider struct {
	c <-chan []byte
}

func (*XWindowClipBoardProvider) Write(b []byte) {
	clipboard.Write(clipboard.FmtText, b)
}

func (x *XWindowClipBoardProvider) Chan() <-chan []byte {
	return x.c
}

func GetProvider() ClipBoardProvider {
	lock.Lock()
	defer lock.Unlock()
	if provider == nil {
		provider = &XWindowClipBoardProvider{
			c: clipboard.Watch(context.TODO(), clipboard.FmtText),
		}
	}
	return provider
}
