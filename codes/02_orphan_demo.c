// orphan_demo.c

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    pid_t pid = fork();

    if (pid < 0) {
        perror("fork failed");
        exit(1);
    }

    if (pid == 0) {
        // Child process
        sleep(5);  // Delay so parent can exit
        printf("Child (PID: %d), my new parent is PID: %d\n", getpid(), getppid());
    } else {
        // Parent process
        printf("Parent (PID: %d) exiting...\n", getpid());
        exit(0);  // Exits immediately — child becomes orphan
    }

    return 0;
}
