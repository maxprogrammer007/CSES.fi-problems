// Problem Statement - https://cses.fi/problemset/task/1068

#include<stdio.h>

int main () {
    int n;
    scanf("%d",&n);
    
    if(n <= 0){
        printf("The given is invalid  ");
        int i;
        printf("Re-enter the number ");
        scanf("%d",&i);
        n = i;
    }
    
    while(n!=1){
        if(n % 2 == 0)
        n = n/2;
        
        else
        n = (n*3) + 1;
        
        printf("%d ",n);
    }
}