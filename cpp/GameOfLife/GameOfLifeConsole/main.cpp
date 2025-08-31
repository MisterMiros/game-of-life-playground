#include <iostream>
#include "console_runner.h"

int main(int argc, char* argv[])
{
    try
    {
        auto runner = console_runner();
        runner.run();
    }
    catch (const std::exception& e)
    {
        std::cerr << e.what() << '\n';
        return 1;
    }
    return 0;
}
