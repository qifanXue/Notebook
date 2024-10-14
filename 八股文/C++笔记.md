# C++基础
[TOC]
## 指针与引用
- 指针本身是变量，存放某个对象的地址
- 引用是变量的别名，不可变，必须初始化
![alt text](image.png)
```C++{.line-numbers}
int main(){
    int x = 10, y = 20;
    int *ptr;
    int &ref = x;//引用必须初始化
    ptr = &x;//ptr存放x的地址
    cout << "ptr的地址是 " << &ptr << endl;//0x16fdff1f0
    cout << "ptr的内容是 " << ptr << endl;//0x16fdff1fc
    cout << "x的地址是 " << &x <<endl;//0x16fdff1fc
    cout << "ptr指向的内容是 " << *ptr << endl;//10
    *ptr = 100;
    cout << "x的值为 " << x << endl;//100
    ptr = &y;
    cout << "ptr的地址是 " << &ptr << endl;//0x16fdff1f0
    cout << "ptr的内容是 " << ptr << endl;//0x16fdff1f8
    cout << "y的地址是 " << &y <<endl;//0x16fdff1f8
    cout << "ptr指向的内容是 " << *ptr << endl;//20
}
```
![alt text](image-1.png)
```C++{.line-numbers}
int main(){
    int x = 10, y = 20;
    // int &ref;报错
    int &ref = x;//引用必须初始化
    cout << "x的地址是 " << &x << endl;//0x16fdff1fc
    cout << "ref的地址是 " << &ref << endl;//0x16fdff1fc
    cout << "ref的值是 " << ref << endl;//10
    ref = y;
    cout << "x的值是 " << x << endl;//20
    //&ref = y;报错，引用的对象不可更改
}
```
用途：
- 指针： 通常用于动态内存分配、数组操作以及函数参数传递。
- 引用： 通常用于函数参数传递、操作符重载以及创建别名。

## 数据类型
- 数据类型的长度
$short(至少16位) \leq int \leq long(至少32位) \leq longlong$ 
- 计算数据类型的大小
```C++
int main(){
    cout << "short is " << sizeof(short) << " bytes.\n";//2
    cout << "int is " << sizeof(int) << " bytes.\n";//4
    cout << "long is " << sizeof(long) << " bytes.\n";//8
    cout << "long long is " << sizeof(long long) << " bytes.\n";//8
}
```
- 符号常量 引入头文件<climits>
```C++
#include<iostream>
#include<climits>

using namespace std;

int main(){
    cout << "INT_MAX is " << INT_MAX << endl;//2147483647
    cout << "INT_MIN is " << INT_MIN << endl;//-2147483648
}
```
## 关键字
### const关键字
1. 常量指针
强调指针指向的对象不可更改，即不可以通过常量指针来改变常量，但是可以更改指针指向的对象
`const int* a = &temp;`
`int const *a = &temp;`
```C++
int main(){
    int temp = 10, temp1 = 40;
    const int* a = &temp;
    cout << "a的地址是 " << &a << endl;//0x16fa6f280
    cout << "a的内容是 " << a << endl;//0x16fa6f28c
    cout << "a指向的对象是 " << *a << endl;//10
    //*a = 20; 报错，不能通过改变常量指针来改变对象
    temp = 20;
    cout << "a指向的对象是 " << *a << endl;//20
    //a指向的对象可以改变
    a = &temp1;
    cout << "a的地址是 " << &a << endl;//0x16fa6f280
    cout << "a的内容是 " << a << endl;//0x16fa6f288
    cout << "a指向的对象是 " << *a << endl;//40
}
```

2. 指针常量
强调指针指向的对象不可更改，但是可以通过指针改变对象的值
`int* const a = &temp;`
```C++
int main(){
    int temp = 10, temp1 = 40;
    int* const a = &temp; 
    cout << "a的地址是 " << &a << endl;//0x16fa6f280
    cout << "a的内容是 " << a << endl;//0x16fa6f28c
    cout << "temp的地址是 " << &temp << endl;//0x16b1cf28c
    cout << "a指向的对象是 " << *a << endl;//10

    *a = 20;
    cout << "temp的值是 " << temp << endl;//20

    //a = &temp1;报错，a指向的对象不可以改变
}
```
### static关键字
`static`关键字主要用于控制变量及函数的生命周期、作用域以及访问权限。
1. 静态变量
- 在函数内部用`static`关键字修饰的变量称为静态变量
- 静态变量在程序中的整个生命周期内存在，不会因为离开作用域而销毁
- 静态变量可以被用来记录函数调用的次数
```C++
#include<iostream>
using namespace std;

void CountAdd(){
    static int count;//静态变量初始化默认为0
    count ++;
    cout << "count的值是 " << count << endl;
}

int main(){
    CountAdd();//1
    CountAdd();//2
}
```

2. 静态函数
在函数前加`static`关键字，静态函数只能被本文件调用，不能被程序中其他文件调用，所以其他文件可以定义同名的函数
```C++
#include<iostream>
using namespace std;

static void examplefunction(){
    cout << "Function" << endl;
}

int main(){
    examplefunction();//Function
}
```

3. 静态成员变量
- 静态成员变量是在类里声明变量，在数据类型前添加`static`关键字。
- 静态成员变量只能在类外单独定义，以便给它分配存储空间
- 所有类的对象都可以调用静态成员变量，直接使用类名也可以调用
```C++
#include<iostream>
using namespace std;

class example{
public:
    static int examplemember;//在类内声明静态成员变量
};
int example::examplemember = 10;//在类外定义静态成员变量

int main(){
    example classA, classB;
    cout << classA.examplemember << endl;//10
    cout << example::examplemember << endl;//10
    classB.examplemember = 20;
    cout << classB.examplemember << endl;//20
    example::examplemember = 30;
    cout << example::examplemember << endl;//30
}
```


