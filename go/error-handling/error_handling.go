package erratum

func Use(opener ResourceOpener, input string) (err error) {
	var res Resource
	for res, err = opener(); err != nil; res, err = opener() {
		if _, ok := err.(TransientError); !ok {
			return err
		}
	}
	defer res.Close()

	defer func() {
		if r := recover(); r != nil {
			switch e := r.(type) {
			case FrobError:
				res.Defrob(e.defrobTag)
				err = e.inner
			case error:
				err = e
			}
		}
	}()
	res.Frob(input)
	return err
}
