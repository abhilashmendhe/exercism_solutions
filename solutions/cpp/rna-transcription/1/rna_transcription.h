#if !defined(RNA_TRANSCRIPTION_H)
#define RNA_TRANSCRIPTION_H

#include<string>
namespace rna_transcription {
    std::string to_rna(const std::string& str);
    char to_rna(char c);
}  // namespace rna_transcription

#endif // RNA_TRANSCRIPTION_H