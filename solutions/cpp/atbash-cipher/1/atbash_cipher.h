#if !defined(ATBASH_CIPHER_H)
#define ATBASH_CIPHER_H
#include<string>
#include<string_view>
namespace atbash_cipher {
    std::string encode(std::string_view str);
    std::string decode(std::string_view str);
}  // namespace atbash_cipher

#endif // ATBASH_CIPHER_H