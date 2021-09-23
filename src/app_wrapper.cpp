
#include "Common_3/OS/Interfaces/IApp.h"

struct RustAppGlue {
    bool (*init)(void*);
    void (*exit)(void*);
    bool (*load)(void*);
    void (*unload)(void*);
    void (*update)(void*, float);
    void (*draw)(void*);
    const char* (*get_name)(void*);
};

void* app_ptr = nullptr;
RustAppGlue* app_glue = nullptr;

struct AppWrapper: public IApp {
    virtual bool Init() override {
        return app_glue->init(app_ptr);
    }

    virtual void Exit() override {
        app_glue->exit(app_ptr);
    }

    virtual bool Load() override {
        return app_glue->load(app_ptr);
    }

    virtual void Unload() override {
        app_glue->unload(app_ptr);
    }

    virtual void Update(float deltaTime) override {
        app_glue->update(app_ptr, deltaTime);
    }

    virtual void Draw() override {
        app_glue->draw(app_ptr);
    }

    virtual const char* GetName() override {
        return app_glue->get_name(app_ptr);
    }
};

#define main RustAppWrapperMain
DEFINE_APPLICATION_MAIN(AppWrapper)

extern "C" int the_forge_main(void* app, RustAppGlue* glue) {
    app_ptr = app;
    app_glue = glue;
    return RustAppWrapperMain(0, nullptr);
}
