package meetup

import (
	"time"
)

// Define the WeekSchedule type here.
type WeekSchedule byte

const (
	First WeekSchedule = iota
	Second
	Third
	Fourth
	Teenth
	Last
)

func Day(wSched WeekSchedule, wDay time.Weekday, month time.Month, year int) int {
	ws := matchingWeekdaysOfMonth(wDay, month, year)
	d := matchingSchedule(ws, wSched)
	if d == 0 {
		panic("No matching day found")
	}
	return d
}

func matchingSchedule(ws []time.Time, sched WeekSchedule) int {
	switch sched {
	case First:
		return ws[0].Day()
	case Second:
		return ws[1].Day()
	case Third:
		return ws[2].Day()
	case Fourth:
		return ws[3].Day()
	case Teenth:
		for _, d := range ws {
			if d.Day() >= 13 && d.Day() <= 19 {
				return d.Day()
			}
		}
	case Last:
		return ws[len(ws)-1].Day()
	}
	return 0
}

func matchingWeekdaysOfMonth(wDay time.Weekday, month time.Month, year int) []time.Time {
	start := time.Date(year, month, 1, 0, 0, 0, 0, time.UTC)
	end := time.Date(year, month+1, 1, 0, 0, 0, 0, time.UTC)
	var ws []time.Time
	for d := start; d.Before(end); d = d.AddDate(0, 0, 1) {
		if d.Weekday() == wDay {
			ws = append(ws, d)
		}
	}
	return ws
}
