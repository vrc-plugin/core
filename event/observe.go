package event

type Observer interface {
	watch() (Event, error)

}

type Observe struct {

}

func (v *Observe) watch() error {
	return nil
}