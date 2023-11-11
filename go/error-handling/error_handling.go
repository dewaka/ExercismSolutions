package erratum

func Use(opener ResourceOpener, input string) (err error) {
	var res Resource
	for {
		res, err = opener()
		if err == nil {
			break
		}
		if _, ok := err.(TransientError); !ok {
			return err
		}
	}
	defer res.Close()
	defer func() {
		if r := recover(); r != nil {
			if frobErr, ok := r.(FrobError); ok {
				res.Defrob(frobErr.defrobTag)
			}
			err = r.(error)
		}
	}()
	res.Frob(input)
	return err
}
