#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

#define MAX_STUDENTS 1000
#define FILENAME "grades.txt"

typedef struct {
    char id[50];
    char name[50];
    float math;
    float english;
    float physics;
    float average;
    int rank;
} Student;

void saveStudentsToFile(Student students[], int count) {
    FILE *file = fopen(FILENAME, "w");
    if (file == NULL) {
        printf("无法打开文件。\n");
        return;
    }

    fprintf(file, "学号\t\t\t姓名\t\t高等数学\t\t英语\t\t物理\t\t平均成绩\t\t名次\n");
    for (int i = 0; i < count; i++) {
        fprintf(file, "%s\t\t%s\t\t%.2f\t\t\t%.2f\t\t%.2f\t\t%.2f\t\t\t%d\n",
                students[i].id, students[i].name, students[i].math, students[i].english,
                students[i].physics, students[i].average, students[i].rank);
    }

    fclose(file);
}

void insertStudent(Student students[], int *count) {
    if (*count >= MAX_STUDENTS) {
        printf("成绩表已满，无法插入新记录。\n");
        return;
    }

    Student student;

    printf("请输入学生的学号：");
    scanf(" %[^\n]1", student.id);

     // 检查学号中是否存在空格
    for (int i = 0; i < strlen(student.id); i++) {
        if (student.id[i] == ' ') {
            printf("根据中国传媒大学本科生学籍管理规定，学号中不能包含空格，请重新输入。\n");
            return;
        }
    }

    // 检查学号是否已存在
    for (int i = 0; i < *count; i++) {
        if (strcmp(students[i].id, student.id) == 0) {
            printf("根据中国传媒大学本科生学籍管理规定，学号已存在，请重新输入。\n");
            return;
        }
    }
    printf("请输入学生的姓名：");
    scanf(" %[^\n]", student.name);  // 使用带空格的格式字符串读取姓名

    // 检查姓名中是否存在空格
    for (int i = 0; i < strlen(student.name); i++) {
        if (student.name[i] == ' ') {
            printf("根据中国传媒大学本科生学籍管理规定，姓名中不能包含空格，请重新输入。\n");
            return;
        }
    }

   do {
    printf("请输入学生的高等数学成绩：");
    if (scanf("%f", &student.math) != 1) {
        printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须为数字，输入无效，请重新输入数字。\n");
        while (getchar() != '\n'); // 清除输入缓冲区
        continue;
    }
    if (student.math < 0 || student.math > 100) {
        printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须在0到100之间，请重新输入。\n");
    }
} while (student.math < 0 || student.math > 100);

do {
    printf("请输入学生的英语成绩：");
    if (scanf("%f", &student.english) != 1) {
        printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须为数字，输入无效，请重新输入数字。\n");
        while (getchar() != '\n'); // 清除输入缓冲区
        continue;
    }
    if (student.english < 0 || student.english > 100) {
        printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须在0到100之间，请重新输入。\n");
    }
} while (student.english < 0 || student.english > 100);

do {
    printf("请输入学生的物理成绩：");
    if (scanf("%f", &student.physics) != 1) {
        printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须为数字，输入无效，请重新输入数字。\n");
        while (getchar() != '\n'); // 清除输入缓冲区
        continue;
    }
    if (student.physics < 0 || student.physics > 100) {
        printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须在0到100之间，请重新输入。\n");
    }
} while (student.physics < 0 || student.physics > 100);


    student.average = (student.math + student.english + student.physics) / 3.0;
    student.rank = 0;

    students[*count] = student;
    (*count)++;

    printf("学生信息插入成功。\n");

    saveStudentsToFile(students, *count);
}

void deleteStudent(Student students[], int *count) {
    char id[50];
    printf("请输入要删除的学生学号：");
    scanf("%s", id);

    int found = 0;
    for (int i = 0; i < *count; i++) {
        if (strcmp(students[i].id, id) == 0) {
            for (int j = i; j < *count - 1; j++) {
                students[j] = students[j + 1];
            }
            (*count)--;
            found = 1;
            printf("学生信息删除成功。\n");
            break;
        }
    }

    if (!found) {
        printf("未找到对应学号的学生。\n");
    }

    saveStudentsToFile(students, *count);
}

void updateStudent(Student students[], int count) {
    char id[50];
    printf("请输入要修改的学生学号：");
    scanf("%s", id);

    int found = 0;
    for (int i = 0; i < count; i++) {
        if (strcmp(students[i].id, id) == 0) {
            do {
                printf("请输入学生的高等数学成绩：");
                if (scanf("%f", &students[i].math) != 1) {
                    printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须为数字，输入无效，请重新输入数字。\n");
                    while (getchar() != '\n'); // 清除输入缓冲区
                    continue;
                }
                if (students[i].math < 0 || students[i].math > 100) {
                    printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须在0到100之间，请重新输入。\n");
                }
            } while (students[i].math < 0 || students[i].math > 100);

            do {
                printf("请输入学生的英语成绩：");
                if (scanf("%f", &students[i].english) != 1) {
                    printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须为数字，输入无效，请重新输入数字。\n");
                    while (getchar() != '\n'); // 清除输入缓冲区
                    continue;
                }
                if (students[i].english < 0 || students[i].english > 100) {
                    printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须在0到100之间，请重新输入。\n");
                }
            } while (students[i].english < 0 || students[i].english > 100);

            do {
                printf("请输入学生的物理成绩：");
                if (scanf("%f", &students[i].physics) != 1) {
                    printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须为数字，输入无效，请重新输入数字。\n");
                    while (getchar() != '\n'); // 清除输入缓冲区
                    continue;
                }
                if (students[i].physics < 0 || students[i].physics > 100) {
                    printf("根据中国传媒大学本科学生课程成绩管理办法（修订），成绩必须在0到100之间，请重新输入。\n");
                }
            } while (students[i].physics < 0 || students[i].physics > 100);

            students[i].average = (students[i].math + students[i].english + students[i].physics) / 3.0;

            printf("学生信息修改成功。\n");
            found = 1;
            break;
        }
    }

    if (!found) {
        printf("未找到对应学号的学生。\n");
    }

    saveStudentsToFile(students, count);
}


/*int compareByMath(const void *a, const void *b) {
    const Student *studentA = (const Student *)a;
    const Student *studentB = (const Student *)b;

    if (studentA->math < studentB->math) {
        return -1;
    } else if (studentA->math > studentB->math) {
        return 1;
    } else {
        return 0;
    }
}

void sortByMath(Student students[], int count) {
    qsort(students, count, sizeof(Student), compareByMath);
}*/

void calculateAverage(Student students[], int count) {
    for (int i = 0; i < count; i++) {
        students[i].average = (students[i].math + students[i].english + students[i].physics) / 3.0;
    }
}

int compareByAverage(const void *a, const void *b) {
    const Student *studentA = (const Student *)a;
    const Student *studentB = (const Student *)b;

    if (studentA->average < studentB->average) {
        return 1;
    } else if (studentA->average > studentB->average) {
        return -1;
    } else {
        return 0;
    }
}

void sortByAverage(Student students[], int count) {
    qsort(students, count, sizeof(Student), compareByAverage);
}

void assignRanks(Student students[], int count) {
    int rank = 1;
    students[0].rank = rank;

    for (int i = 1; i < count; i++) {
        if (students[i].average == students[i - 1].average) {
            students[i].rank = rank;
        } else {
            students[i].rank = ++rank;
        }
    }
}

void printStudents(Student students[], int count) {
    printf("学号\t\t\t姓名\t\t高等数学\t\t英语\t\t物理\t\t平均成绩\t\t名次\n");
    for (int i = 0; i < count; i++) {
        printf("%13s\t\t%s\t\t%.2f\t\t\t%.2f\t\t%.2f\t\t%.2f\t\t\t%d\n",
               students[i].id, students[i].name, students[i].math, students[i].english,
               students[i].physics, students[i].average, students[i].rank);
    }
}

void readStudentsFromFile(Student students[], int *count) {
    FILE *file = fopen(FILENAME, "r");
    if (file == NULL) {
        printf("无法打开文件。\n");
        return;
    }

    fscanf(file, "%*[^\n]\n");  // 跳过表头

    while (fscanf(file, "%s %s %f %f %f %f %d",
                  students[*count].id, students[*count].name, &students[*count].math,
                  &students[*count].english, &students[*count].physics,
                  &students[*count].average, &students[*count].rank) == 7) {
        (*count)++;
    }

    fclose(file);
}

int main() {
    Student students[MAX_STUDENTS];
    int count = 0;

    readStudentsFromFile(students, &count);

    char choice;
    do {
        printf("\n成绩管理程序\n");
        printf("1. 插入学生记录\n");
        printf("2. 删除学生记录\n");
        printf("3. 修改学生记录\n");
        printf("4. 开发中,无法使用\n");
        printf("5. 计算每名学生的平均成绩并且输出成绩表\n");
        printf("0. 退出程序\n");
        printf("请输入选项（只允许输入数字，否则后果自负）：");
        scanf("%c", &choice);

        switch (choice) {
            case '1':
                insertStudent(students, &count);
                break;
            case '2':
                deleteStudent(students, &count);
                break;
            case '3':
                updateStudent(students, count);
                break;
            /*case 4:
                sortByMath(students, count);
                break;*/
            case '5':
                calculateAverage(students, count);
                sortByAverage(students, count);
                assignRanks(students, count);
                printStudents(students, count);
                saveStudentsToFile(students, count);
                break;
            case '0':
                printf("程序已退出。\n");
                break;
            default:
                printf("无效的选项。\n");
                break;
        }
    } while (choice != 0);

    return 0;
}
