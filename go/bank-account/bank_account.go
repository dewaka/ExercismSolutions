package account

import (
	"sync"
)

type Account struct {
	sync.RWMutex
	balance int64
	closed  bool
}

func Open(amount int64) *Account {
	if amount < 0 {
		return nil
	}
	return &Account{balance: amount, closed: false}
}

func (a *Account) Balance() (int64, bool) {
	a.RLock()
	defer a.RUnlock()
	if a.closed {
		return 0, false
	}
	if a.balance < 0 {
		return a.balance, false
	}
	return a.balance, true
}

func (a *Account) Deposit(amount int64) (int64, bool) {
	a.Lock()
	defer a.Unlock()
	if a.closed {
		return a.balance, false
	}
	newBalance := a.balance + amount
	if newBalance < 0 {
		return a.balance, false
	}
	a.balance = newBalance
	return a.balance, true
}

func (a *Account) Close() (int64, bool) {
	a.Lock()
	defer a.Unlock()
	if a.closed {
		return 0, false
	}
	a.closed = true
	return a.balance, true
}
