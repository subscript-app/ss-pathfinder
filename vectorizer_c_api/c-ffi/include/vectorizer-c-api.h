/* Generated code. Do not edit; instead run `cargo build` in `pathfinder_c`. */

#ifndef PF_PATHFINDER_H
#define PF_PATHFINDER_H

#if defined(__APPLE__) && defined(__OBJC__)
#include <QuartzCore/QuartzCore.h>
#else
typedef struct CAMetalLayerPrivate CAMetalLayer;
#endif

#ifdef __cplusplus
extern "C" {
#endif


/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define PF_LINE_CAP_BUTT 0

#define PF_LINE_CAP_SQUARE 1

#define PF_LINE_CAP_ROUND 2

#define PF_LINE_JOIN_MITER 0

#define PF_LINE_JOIN_BEVEL 1

#define PF_LINE_JOIN_ROUND 2

#define PF_TEXT_ALIGN_LEFT 0

#define PF_TEXT_ALIGN_CENTER 1

#define PF_TEXT_ALIGN_RIGHT 2

#define PF_TEXT_BASELINE_ALPHABETIC 0

#define PF_TEXT_BASELINE_TOP 1

#define PF_TEXT_BASELINE_HANGING 2

#define PF_TEXT_BASELINE_MIDDLE 3

#define PF_TEXT_BASELINE_IDEOGRAPHIC 4

#define PF_TEXT_BASELINE_BOTTOM 5

#define PF_ARC_DIRECTION_CW 0

#define PF_ARC_DIRECTION_CCW 1

#define PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR 1

#define PF_RENDERER_OPTIONS_FLAGS_SHOW_DEBUG_UI 2

#define PF_RENDERER_LEVEL_D3D9 1

#define PF_RENDERER_LEVEL_D3D11 2

/**
 * Options that influence scene building.
 */
typedef struct PFBuildOptionsPrivate PFBuildOptionsPrivate;

#if !defined(PATHFINDER_TEXT)
typedef struct PFCanvasFontContextPrivate PFCanvasFontContextPrivate;
#endif

typedef struct PFCanvasRenderingContext2DPrivate PFCanvasRenderingContext2DPrivate;

/**
 * Where the rendered content should go.
 */
typedef struct PFDestFramebufferMetalDevicePrivate PFDestFramebufferMetalDevicePrivate;

typedef struct PFFillStylePrivate PFFillStylePrivate;

typedef struct PFMetalDevicePrivate PFMetalDevicePrivate;

typedef struct PFPath2DPrivate PFPath2DPrivate;

/**
 * A global transform to apply to the scene.
 */
typedef struct PFRenderTransformPrivate PFRenderTransformPrivate;

/**
 * The GPU renderer that processes commands necessary to render a scene.
 */
typedef struct PFRendererMetalDevicePrivate PFRendererMetalDevicePrivate;

typedef struct PFResourceLoaderWrapperPrivate PFResourceLoaderWrapperPrivate;

/**
 * The vector scene to be rendered.
 */
typedef struct PFScenePrivate PFScenePrivate;

/**
 * A version of `Scene` that proxies all method calls out to a separate thread.
 */
typedef struct PFSceneProxyPrivate PFSceneProxyPrivate;

typedef struct PFCanvasRenderingContext2DPrivate *PFCanvasRef;

typedef struct PFCanvasFontContextPrivate *PFCanvasFontContextRef;

typedef struct PFVector2F {
  float x;
  float y;
} PFVector2F;

typedef struct PFScenePrivate *PFSceneRef;

typedef struct PFRectF {
  struct PFVector2F origin;
  struct PFVector2F lower_right;
} PFRectF;

typedef uint8_t PFLineCap;

typedef uint8_t PFLineJoin;

/**
 * Row-major order.
 */
typedef struct PFMatrix2x2F {
  float m00;
  float m01;
  float m10;
  float m11;
} PFMatrix2x2F;

/**
 * Row-major order.
 */
typedef struct PFTransform2F {
  struct PFMatrix2x2F matrix;
  struct PFVector2F vector;
} PFTransform2F;

typedef struct PFFillStylePrivate *PFFillStyleRef;

typedef struct PFPath2DPrivate *PFPathRef;

typedef uint8_t PFArcDirection;

typedef struct PFColorU {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} PFColorU;

typedef struct PFResourceLoaderWrapperPrivate *PFResourceLoaderRef;

typedef struct PFDestFramebufferMetalDevicePrivate *PFMetalDestFramebufferRef;

typedef struct PFVector2I {
  int32_t x;
  int32_t y;
} PFVector2I;

typedef struct PFRendererMetalDevicePrivate *PFMetalRendererRef;

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
typedef struct PFMetalDevicePrivate *PFMetalDeviceRef;
#endif

typedef uint8_t PFRendererLevel;

typedef struct PFRendererMode {
  PFRendererLevel level;
} PFRendererMode;

typedef void *PFDestFramebufferRef;

typedef struct PFColorF {
  float r;
  float g;
  float b;
  float a;
} PFColorF;

typedef uint8_t PFRendererOptionsFlags;

typedef struct PFRendererOptions {
  PFDestFramebufferRef dest;
  struct PFColorF background_color;
  PFRendererOptionsFlags flags;
} PFRendererOptions;

typedef struct PFSceneProxyPrivate *PFSceneProxyRef;

typedef struct PFBuildOptionsPrivate *PFBuildOptionsRef;

typedef struct PFRenderTransformPrivate *PFRenderTransformRef;

/**
 * Row-major order.
 */
typedef struct PFTransform4F {
  float m00;
  float m01;
  float m02;
  float m03;
  float m10;
  float m11;
  float m12;
  float m13;
  float m20;
  float m21;
  float m22;
  float m23;
  float m30;
  float m31;
  float m32;
  float m33;
} PFTransform4F;

typedef struct PFPerspective {
  struct PFTransform4F transform;
  struct PFVector2I window_size;
} PFPerspective;

/**
 * This function internally adds a reference to the font context. Therefore, if you created the
 * font context, you must release it yourself to avoid a leak.
 */
PFCanvasRef PFCanvasCreate(PFCanvasFontContextRef font_context, const struct PFVector2F *size);

void PFCanvasDestroy(PFCanvasRef canvas);

PFCanvasFontContextRef PFCanvasFontContextCreateWithSystemSource(void);

/**
 * This function takes ownership of the supplied canvas and will automatically destroy it when
 * the scene is destroyed.
 */
PFSceneRef PFCanvasCreateScene(PFCanvasRef canvas);

void PFCanvasFillRect(PFCanvasRef canvas, const struct PFRectF *rect);

void PFCanvasStrokeRect(PFCanvasRef canvas, const struct PFRectF *rect);

void PFCanvasSetLineWidth(PFCanvasRef canvas, float new_line_width);

void PFCanvasSetLineCap(PFCanvasRef canvas, PFLineCap new_line_cap);

void PFCanvasSetLineJoin(PFCanvasRef canvas, PFLineJoin new_line_join);

void PFCanvasSetMiterLimit(PFCanvasRef canvas, float new_miter_limit);

void PFCanvasSetLineDash(PFCanvasRef canvas,
                         const float *new_line_dashes,
                         uintptr_t new_line_dash_count);

void PFCanvasSetTransform(PFCanvasRef canvas, const struct PFTransform2F *transform);

void PFCanvasResetTransform(PFCanvasRef canvas);

void PFCanvasSave(PFCanvasRef canvas);

void PFCanvasRestore(PFCanvasRef canvas);

void PFCanvasSetLineDashOffset(PFCanvasRef canvas, float new_offset);

void PFCanvasSetFillStyle(PFCanvasRef canvas, PFFillStyleRef fill_style);

void PFCanvasSetStrokeStyle(PFCanvasRef canvas, PFFillStyleRef stroke_style);

/**
 * This function automatically destroys the path. If you wish to use the path again, clone it
 * first.
 */
void PFCanvasFillPath(PFCanvasRef canvas, PFPathRef path);

/**
 * This function automatically destroys the path. If you wish to use the path again, clone it
 * first.
 */
void PFCanvasStrokePath(PFCanvasRef canvas, PFPathRef path);

PFPathRef PFPathCreate(void);

void PFPathDestroy(PFPathRef path);

PFPathRef PFPathClone(PFPathRef path);

void PFPathMoveTo(PFPathRef path, const struct PFVector2F *to);

void PFPathLineTo(PFPathRef path, const struct PFVector2F *to);

void PFPathQuadraticCurveTo(PFPathRef path,
                            const struct PFVector2F *ctrl,
                            const struct PFVector2F *to);

void PFPathBezierCurveTo(PFPathRef path,
                         const struct PFVector2F *ctrl0,
                         const struct PFVector2F *ctrl1,
                         const struct PFVector2F *to);

void PFPathArc(PFPathRef path,
               const struct PFVector2F *center,
               float radius,
               float start_angle,
               float end_angle,
               PFArcDirection direction);

void PFPathArcTo(PFPathRef path,
                 const struct PFVector2F *ctrl,
                 const struct PFVector2F *to,
                 float radius);

void PFPathRect(PFPathRef path, const struct PFRectF *rect);

void PFPathEllipse(PFPathRef path,
                   const struct PFVector2F *center,
                   const struct PFVector2F *axes,
                   float rotation,
                   float start_angle,
                   float end_angle);

void PFPathClosePath(PFPathRef path);

PFFillStyleRef PFFillStyleCreateColor(const struct PFColorU *color);

void PFFillStyleDestroy(PFFillStyleRef fill_style);

PFResourceLoaderRef PFEmbeddedResourceLoaderCreate(void);

PFResourceLoaderRef PFFilesystemResourceLoaderLocate(void);

PFResourceLoaderRef PFFilesystemResourceLoaderFromPath(const char *path);

void PFResourceLoaderDestroy(PFResourceLoaderRef loader);

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
PFMetalDestFramebufferRef PFMetalDestFramebufferCreateFullWindow(const struct PFVector2I *window_size);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
void PFMetalDestFramebufferDestroy(PFMetalDestFramebufferRef dest_framebuffer);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
/**
 * This function takes ownership of and automatically takes responsibility for destroying `device`
 * and `dest_framebuffer`. However, it does not take ownership of `resources`; therefore, if you
 * created the resource loader, you must destroy it yourself to avoid a memory leak.
 */
PFMetalRendererRef PFMetalRendererCreate(PFMetalDeviceRef device,
                                         PFResourceLoaderRef resources,
                                         const struct PFRendererMode *mode,
                                         const struct PFRendererOptions *options);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
void PFMetalRendererDestroy(PFMetalRendererRef renderer);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
/**
 * Returns a reference to the Metal device in the renderer.
 *
 * This reference remains valid as long as the device is alive.
 */
PFMetalDeviceRef PFMetalRendererGetDevice(PFMetalRendererRef renderer);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
/**
 * This function does not take ownership of `renderer` or `build_options`. Therefore, if you
 * created the renderer and/or options, you must destroy them yourself to avoid a leak.
 */
void PFSceneProxyBuildAndRenderMetal(PFSceneProxyRef scene_proxy,
                                     PFMetalRendererRef renderer,
                                     PFBuildOptionsRef build_options);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
PFMetalDeviceRef PFMetalDeviceCreateWithIOSurface(const NSObject<MTLDevice> *metal_device,
                                                  IOSurfaceRef io_surface);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
PFMetalDeviceRef PFMetalDeviceCreateWithDrawable(const NSObject<MTLDevice> *metal_device,
                                                 const NSObject<CAMetalDrawable> *ca_drawable);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
void PFMetalDeviceSwapIOSurface(PFMetalDeviceRef device, IOSurfaceRef new_io_surface);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
void PFMetalDeviceSwapDrawable(PFMetalDeviceRef device,
                               const NSObject<CAMetalDrawable> *new_ca_drawable);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
void PFMetalDevicePresentDrawable(PFMetalDeviceRef device,
                                  const NSObject<CAMetalDrawable> *ca_drawable);
#endif

#if (defined(__APPLE__) && !defined(PATHFINDER_GL))
void PFMetalDeviceDestroy(PFMetalDeviceRef device);
#endif

PFRenderTransformRef PFRenderTransformCreate2D(const struct PFTransform2F *transform);

PFRenderTransformRef PFRenderTransformCreatePerspective(const struct PFPerspective *perspective);

void PFRenderTransformDestroy(PFRenderTransformRef transform);

PFBuildOptionsRef PFBuildOptionsCreate(void);

void PFBuildOptionsDestroy(PFBuildOptionsRef options);

/**
 * Consumes the transform.
 */
void PFBuildOptionsSetTransform(PFBuildOptionsRef options, PFRenderTransformRef transform);

void PFBuildOptionsSetDilation(PFBuildOptionsRef options, const struct PFVector2F *dilation);

void PFBuildOptionsSetSubpixelAAEnabled(PFBuildOptionsRef options, bool subpixel_aa_enabled);

void PFSceneDestroy(PFSceneRef scene);

PFSceneProxyRef PFSceneProxyCreateFromSceneAndRayonExecutor(PFSceneRef scene,
                                                            PFRendererLevel level);

void PFSceneProxyDestroy(PFSceneProxyRef scene_proxy);

#ifdef __cplusplus
}
#endif

#endif
