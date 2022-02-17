class Foo {
private:
    volatile int counter;
public:
    Foo() {
        counter = 0;
    }

    void first(function<void()> printFirst) {
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        counter = 1;
    }

    void second(function<void()> printSecond) {
        while (counter < 1);
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        counter = 2;
    }

    void third(function<void()> printThird) {
        while (counter < 2);
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
        counter = 3;
    }
};