// Convert JSON text to a Person object.
func Deserialize(a,b int) (Person, error) {
}

func test() []int {
}

func testVariadic() int {
}

func testQualifiedType() package.MyType {
}

func testPointerType() *Type {
}

func testStructType() struct{ Name string } {
}

func testArrayType() [10]int {
}

func testSliceType() []int {
}

func testMapType() map[string]int {
}

func testChannelType() <- chan int {
}

// This is not correct Go!
func testUnionType() int | string {
}

// This is also not correct Go!
func testNegatedType() ~int {
}

func testNegatedType() interface{ ~int } {
}

func noReturnValue() {
}