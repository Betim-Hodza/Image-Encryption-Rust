#include <iostream>
#include <opencv4/opencv2/opencv.hpp>
#include <cryptopp/cryptlib.h>
#include <cryptopp/aes.h>
#include <cryptopp/modes.h>
#include <cryptopp/osrng.h>


int main()
{
    //test open cv
    cv::Mat image = cv::imread("HackerCat.jpg", cv::IMREAD_COLOR);
    if(image.empty())
    {
        std::cerr << "Failed to read image" << std::endl;
        return -1;
    }
    else
    {
        std::cout << "Succussfully read the image!" << std::endl;
    }

    // test crypto ++
    CryptoPP::AutoSeededRandomPool rng;
    CryptoPP::byte key[CryptoPP::AES::DEFAULT_KEYLENGTH];
    rng.GenerateBlock(key, sizeof(key));
    std::cout << "Generated AES key." <<std::endl;
}