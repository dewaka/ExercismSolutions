package allergies

var allergens = []string{"eggs", "peanuts", "shellfish", "strawberries", "tomatoes", "chocolate", "pollen", "cats"}

func Allergies(allergies uint) []string {
	a := []string{}
	for i := 0; i < len(allergens) && allergies != 0; i, allergies = i+1, allergies>>1 {
		if allergies&1 == 1 {
			a = append(a, allergens[i])
		}
	}
	return a
}

func AllergicTo(allergies uint, allergen string) bool {
	for i, a := range allergens {
		if a == allergen {
			return allergies&(1<<i) != 0
		}
	}
	return false
}
