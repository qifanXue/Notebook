#include<stdio.h>

/* -------------------------------------------------------------------------- */
/*                       输入n是2的幂次，b是小于n的整数，求b关于n比特逆转后的值                       */
/* -------------------------------------------------------------------------- */
int bitreversal(int n, int b){
    int log_n = 0, c = 0;
    //计算n的二进制位
    while(n>1){
        n >>= 1;
        log_n++;
    }
    for(int i = 0;i < log_n;i ++){
        c = (c << 1)|(b & 1);
        b >>= 1;
    }
    return c;
}


int main(){
    int n = 8;
    int b = 3;
    int brv = bitreversal(n,b);
    printf("%d",brv);
    return 0;
}