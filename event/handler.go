package event

type Handle struct {
	observe Observe
}

type Handler interface {
}

func (h *Handle) Dispatch() {

}
