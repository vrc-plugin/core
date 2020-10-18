package event

import "os"

type Observer interface {
	watch() (Event, error)
}

type Observe struct {
	logDir     os.FileInfo
	currentLog os.FileInfo
}

func NewObserver(logDir os.FileInfo) *Observe {
	return &Observe{
		logDir:     logDir,
		currentLog: nil,
	}
}

func (v *Observe) processWatch() error {

	for {
	}
	return nil

}

func (v *Observe) dirWatch() error {

	for {
	}

	return nil
}

func (v *Observe) logWatch() error {
	for {
	}

	return nil
}
