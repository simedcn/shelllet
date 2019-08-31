#include "mainwindow.h"
#include <QApplication>
#include <QDir>
int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    QDir::home().mkpath("shelllet.com");

    MainWindow w;
    w.show();


    return a.exec();
}
