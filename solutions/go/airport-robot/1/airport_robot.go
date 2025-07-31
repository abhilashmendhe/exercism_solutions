package airportrobot

// Write your code here.
// This exercise does not have tests for each individual task.
// Try to solve all the tasks first before running the tests.
type Greeter interface {
	LanguageName() string
	Greet(name string) string
}
type German struct {
}
type Italian struct {
}
type Portuguese struct {
}

func (g German) LanguageName() string {
	return "I can speak German: Hallo "
}
func (g German) Greet(name string) string {
	return g.LanguageName() + name + "!"
}
func (it Italian) LanguageName() string {
	return "I can speak Italian: Ciao "
}
func (it Italian) Greet(name string) string {
	return it.LanguageName() + name + "!"
}
func (pr Portuguese) LanguageName() string {
	return "I can speak Portuguese: Olá "
}
func (pr Portuguese) Greet(name string) string {
	return pr.LanguageName() + name + "!"
}

func SayHello(name string, gr Greeter) string {
	return gr.Greet(name)
}
