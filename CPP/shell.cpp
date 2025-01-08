#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <windows.h>
#include <Lmcons.h>
#include <filesystem>

namespace fs = std::filesystem;


enum Commands
{
    CAT,
    ECHO,
    LS,
    EXIT
};

std::string getCommandAndArgument(std::string command, Commands &commandPointer)
{
    std::string helper;
    bool isLong;
    for(int i = 0; i < command.size(); i++){
        if(command[i] == ' '&& !isLong){
            if (helper == "echo" )
            {
                commandPointer = ECHO;
            }else if(helper == "cat"){
                commandPointer = CAT;
            }else if(helper == "ls"){
                commandPointer = LS;
            }else{
                commandPointer = EXIT;
            }
            helper = "";
        }else{
            if(command[i] == '"'){
                isLong = !isLong;
            }else{
                helper += command[i];
            }
        }
    }
    if (helper == "exit") commandPointer = EXIT;
    return helper;
}

void showFileContent(std::string path)
{
    std::ifstream MyReadFile(path);
    std::string toShow;
    while(std::getline (MyReadFile, toShow)){
        std::cout << toShow << '\n';
    }
    MyReadFile.close();
    return;
}

void listDirectory(std::string path)
{  
    std::string toShow;
    for (const auto & entry: fs::directory_iterator(path)){
        
        std::cout << entry.path() << std::endl;
    }
}



int main()
{
    Commands command;
    std::string userCommand;
    std::string commandArgument;
    TCHAR username[UNLEN+1];
    DWORD username_len = UNLEN+1;
    GetUserName(username, &username_len);
    std::cout << "Welcome to C++ Terminal!!\n";
    do{
        std::wcout 
            << username 
            << ">";

        std::getline(std::cin, userCommand);
        commandArgument = getCommandAndArgument(userCommand, command);
        switch (command)
        {
        case ECHO:
            std::cout << commandArgument << '\n';
            break;
        case CAT:
            showFileContent(commandArgument);
            break;
        case LS:
            listDirectory(commandArgument);
            break;
        default:
            break;
        }
    }while(command != EXIT);
    std::cout << "See you next time!!" << '\n';


    return 0;
}




