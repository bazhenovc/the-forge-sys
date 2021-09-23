
#include "Common_3/OS/Interfaces/IFileSystem.h"

extern "C" {

// IFileSystem

FileMode wrap_fsFileModeFromString(const char* modeStr) {
	return fsFileModeFromString(modeStr);
}

const char* wrap_fsFileModeToString(FileMode mode) {
	return fsFileModeToString(mode);
}

const char* wrap_fsOverwriteFileModeToString(FileMode mode) {
	return fsOverwriteFileModeToString(mode);
}

}
