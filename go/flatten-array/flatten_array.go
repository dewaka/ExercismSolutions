package flatten

func Flatten(nested interface{}) []interface{} {
	res := make([]interface{}, 0)
	if nested == nil {
		return res
	}

	switch v := nested.(type) {
	case []interface{}:
		for _, item := range v {
			flattened := Flatten(item)
			if flattened != nil {
				res = append(res, flattened...)
			}
		}
	default:
		res = append(res, v)
	}

	return res
}
