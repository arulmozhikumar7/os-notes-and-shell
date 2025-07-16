#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>

int main(){
    pid_t pid = fork();

    if(pid<0){
        perror("fork failed");
        exit(1);
    }

    if(pid==0){
        // Child Process
        printf("Child (PID: %d) exiting...\n", getpid());
        exit(0); // exits immediately
    }else{
        // Parent process
        printf("Parent (PID: %d), sleeping for 30 seconds...\n", getpid());
        sleep(30); // doesn't call wait(), child becomes zombie
        // Fix : change sleep(30) to wait(NULL)
        wait(NULL);

        printf("Parent done sleeping.\n");
    }

    return 0;

}