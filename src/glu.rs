use core::libc::*;
use gl::*;

pub type GLUenum = c_uint;
pub type GLUfloat = c_float;

type voidstar = u8;
type nullary_void_callback = *u8;

/* Extensions */
pub const GLU_EXT_object_space_tess : GLUenum =            1;
pub const GLU_EXT_nurbs_tessellator : GLUenum =            1;

/* Boolean */
pub const GLU_FALSE : GLUenum =                            0;
pub const GLU_TRUE : GLUenum =                             1;

/* Version */
pub const GLU_VERSION_1_1 : GLUenum =                      1;
pub const GLU_VERSION_1_2 : GLUenum =                      1;
pub const GLU_VERSION_1_3 : GLUenum =                      1;

/* StringName */
pub const GLU_VERSION : GLUenum =                          100800;
pub const GLU_EXTENSIONS : GLUenum =                       100801;

/* ErrorCode */
pub const GLU_INVALID_ENUM : GLUenum =                     100900;
pub const GLU_INVALID_VALUE : GLUenum =                    100901;
pub const GLU_OUT_OF_MEMORY : GLUenum =                    100902;
pub const GLU_INCOMPATIBLE_GL_VERSION : GLUenum =          100903;
pub const GLU_INVALID_OPERATION : GLUenum =                100904;


/* NurbsDisplay */
/*      GLU_FILL */
pub const GLU_OUTLINE_POLYGON : GLUenum =                  100240;
pub const GLU_OUTLINE_PATCH : GLUenum =                    100241;

/* NurbsCallback */
pub const GLU_NURBS_ERROR : GLUenum =                      100103;
pub const GLU_ERROR : GLUenum =                            100103;
pub const GLU_NURBS_BEGIN : GLUenum =                      100164;
pub const GLU_NURBS_BEGIN_EXT : GLUenum =                  100164;
pub const GLU_NURBS_VERTEX : GLUenum =                     100165;
pub const GLU_NURBS_VERTEX_EXT : GLUenum =                 100165;
pub const GLU_NURBS_NORMAL : GLUenum =                     100166;
pub const GLU_NURBS_NORMAL_EXT : GLUenum =                 100166;
pub const GLU_NURBS_COLOR : GLUenum =                      100167;
pub const GLU_NURBS_COLOR_EXT : GLUenum =                  100167;
pub const GLU_NURBS_TEXTURE_COORD : GLUenum =              100168;
pub const GLU_NURBS_TEX_COORD_EXT : GLUenum =              100168;
pub const GLU_NURBS_END : GLUenum =                        100169;
pub const GLU_NURBS_END_EXT : GLUenum =                    100169;
pub const GLU_NURBS_BEGIN_DATA : GLUenum =                 100170;
pub const GLU_NURBS_BEGIN_DATA_EXT : GLUenum =             100170;
pub const GLU_NURBS_VERTEX_DATA : GLUenum =                100171;
pub const GLU_NURBS_VERTEX_DATA_EXT : GLUenum =            100171;
pub const GLU_NURBS_NORMAL_DATA : GLUenum =                100172;
pub const GLU_NURBS_NORMAL_DATA_EXT : GLUenum =            100172;
pub const GLU_NURBS_COLOR_DATA : GLUenum =                 100173;
pub const GLU_NURBS_COLOR_DATA_EXT : GLUenum =             100173;
pub const GLU_NURBS_TEXTURE_COORD_DATA : GLUenum =         100174;
pub const GLU_NURBS_TEX_COORD_DATA_EXT : GLUenum =         100174;
pub const GLU_NURBS_END_DATA : GLUenum =                   100175;
pub const GLU_NURBS_END_DATA_EXT : GLUenum =               100175;

/* NurbsError */
pub const GLU_NURBS_ERROR1 : GLUenum =                     100251;   /* spline order un-supported */
pub const GLU_NURBS_ERROR2 : GLUenum =                     100252;   /* too few knots */
pub const GLU_NURBS_ERROR3 : GLUenum =                     100253;   /* valid knot range is empty */
pub const GLU_NURBS_ERROR4 : GLUenum =                     100254;   /* decreasing knot sequence */
pub const GLU_NURBS_ERROR5 : GLUenum =                     100255;   /* knot multiplicity > spline order */
pub const GLU_NURBS_ERROR6 : GLUenum =                     100256;   /* endcurve() must follow bgncurve() */
pub const GLU_NURBS_ERROR7 : GLUenum =                     100257;   /* bgncurve() must precede endcurve() */
pub const GLU_NURBS_ERROR8 : GLUenum =                     100258;   /* ctrlarray or knot vector is NULL */
pub const GLU_NURBS_ERROR9 : GLUenum =                     100259;   /* can't draw pwlcurves */
pub const GLU_NURBS_ERROR10 : GLUenum =                    100260;   /* missing gluNurbsCurve() */
pub const GLU_NURBS_ERROR11 : GLUenum =                    100261;   /* missing gluNurbsSurface() */
pub const GLU_NURBS_ERROR12 : GLUenum =                    100262;   /* endtrim() must precede endsurface() */
pub const GLU_NURBS_ERROR13 : GLUenum =                    100263;   /* bgnsurface() must precede endsurface() */
pub const GLU_NURBS_ERROR14 : GLUenum =                    100264;   /* curve of improper type passed as trim curve */
pub const GLU_NURBS_ERROR15 : GLUenum =                    100265;   /* bgnsurface() must precede bgntrim() */
pub const GLU_NURBS_ERROR16 : GLUenum =                    100266;   /* endtrim() must follow bgntrim() */
pub const GLU_NURBS_ERROR17 : GLUenum =                    100267;   /* bgntrim() must precede endtrim()*/
pub const GLU_NURBS_ERROR18 : GLUenum =                    100268;   /* invalid or missing trim curve*/
pub const GLU_NURBS_ERROR19 : GLUenum =                    100269;   /* bgntrim() must precede pwlcurve() */
pub const GLU_NURBS_ERROR20 : GLUenum =                    100270;   /* pwlcurve referenced twice*/
pub const GLU_NURBS_ERROR21 : GLUenum =                    100271;   /* pwlcurve and nurbscurve mixed */
pub const GLU_NURBS_ERROR22 : GLUenum =                    100272;   /* improper usage of trim data type */
pub const GLU_NURBS_ERROR23 : GLUenum =                    100273;   /* nurbscurve referenced twice */
pub const GLU_NURBS_ERROR24 : GLUenum =                    100274;   /* nurbscurve and pwlcurve mixed */
pub const GLU_NURBS_ERROR25 : GLUenum =                    100275;   /* nurbssurface referenced twice */
pub const GLU_NURBS_ERROR26 : GLUenum =                    100276;   /* invalid property */
pub const GLU_NURBS_ERROR27 : GLUenum =                    100277;   /* endsurface() must follow bgnsurface() */
pub const GLU_NURBS_ERROR28 : GLUenum =                    100278;   /* intersecting or misoriented trim curves */
pub const GLU_NURBS_ERROR29 : GLUenum =                    100279;   /* intersecting trim curves */
pub const GLU_NURBS_ERROR30 : GLUenum =                    100280;   /* UNUSED */
pub const GLU_NURBS_ERROR31 : GLUenum =                    100281;   /* unconnected trim curves */
pub const GLU_NURBS_ERROR32 : GLUenum =                    100282;   /* unknown knot error */
pub const GLU_NURBS_ERROR33 : GLUenum =                    100283;   /* negative vertex count encountered */
pub const GLU_NURBS_ERROR34 : GLUenum =                    100284;   /* negative byte-stride */
pub const GLU_NURBS_ERROR35 : GLUenum =                    100285;   /* unknown type descriptor */
pub const GLU_NURBS_ERROR36 : GLUenum =                    100286;   /* null control point reference */
pub const GLU_NURBS_ERROR37 : GLUenum =                    100287;   /* duplicate point on pwlcurve */

/* NurbsProperty */
pub const GLU_AUTO_LOAD_MATRIX : GLUenum =                 100200;
pub const GLU_CULLING : GLUenum =                          100201;
pub const GLU_SAMPLING_TOLERANCE : GLUenum =               100203;
pub const GLU_DISPLAY_MODE : GLUenum =                     100204;
pub const GLU_PARAMETRIC_TOLERANCE : GLUenum =             100202;
pub const GLU_SAMPLING_METHOD : GLUenum =                  100205;
pub const GLU_U_STEP : GLUenum =                           100206;
pub const GLU_V_STEP : GLUenum =                           100207;
pub const GLU_NURBS_MODE : GLUenum =                       100160;
pub const GLU_NURBS_MODE_EXT : GLUenum =                   100160;
pub const GLU_NURBS_TESSELLATOR : GLUenum =                100161;
pub const GLU_NURBS_TESSELLATOR_EXT : GLUenum =            100161;
pub const GLU_NURBS_RENDERER : GLUenum =                   100162;
pub const GLU_NURBS_RENDERER_EXT : GLUenum =               100162;

/* NurbsSampling */
pub const GLU_OBJECT_PARAMETRIC_ERROR : GLUenum =          100208;
pub const GLU_OBJECT_PARAMETRIC_ERROR_EXT : GLUenum =      100208;
pub const GLU_OBJECT_PATH_LENGTH : GLUenum =               100209;
pub const GLU_OBJECT_PATH_LENGTH_EXT : GLUenum =           100209;
pub const GLU_PATH_LENGTH : GLUenum =                      100215;
pub const GLU_PARAMETRIC_ERROR : GLUenum =                 100216;
pub const GLU_DOMAIN_DISTANCE : GLUenum =                  100217;

/* NurbsTrim */
pub const GLU_MAP1_TRIM_2 : GLUenum =                      100210;
pub const GLU_MAP1_TRIM_3 : GLUenum =                      100211;

/* QuadricDrawStyle */ 
pub const GLU_POINT : GLUenum =                            100010;
pub const GLU_LINE : GLUenum =                             100011;
pub const GLU_FILL : GLUenum =                             100012;
pub const GLU_SILHOUETTE : GLUenum =                       100013;
  
/* QuadricCallback */
/*      GLU_ERROR */

/* QuadricNormal */
pub const GLU_SMOOTH : GLUenum =                           100000;
pub const GLU_FLAT : GLUenum =                             100001;
pub const GLU_NONE : GLUenum =                             100002;
 
/* QuadricOrientation */
pub const GLU_OUTSIDE : GLUenum =                          100020;
pub const GLU_INSIDE : GLUenum =                           100021;

/* TessCallback */
pub const GLU_TESS_BEGIN : GLUenum =                       100100;
pub const GLU_BEGIN : GLUenum =                            100100;
pub const GLU_TESS_VERTEX : GLUenum =                      100101;
pub const GLU_VERTEX : GLUenum =                           100101;
pub const GLU_TESS_END : GLUenum =                         100102;
pub const GLU_END : GLUenum =                              100102;
pub const GLU_TESS_ERROR : GLUenum =                       100103;
pub const GLU_TESS_EDGE_FLAG : GLUenum =                   100104;
pub const GLU_EDGE_FLAG : GLUenum =                        100104;
pub const GLU_TESS_COMBINE : GLUenum =                     100105;
pub const GLU_TESS_BEGIN_DATA : GLUenum =                  100106;
pub const GLU_TESS_VERTEX_DATA : GLUenum =                 100107;
pub const GLU_TESS_END_DATA : GLUenum =                    100108;
pub const GLU_TESS_ERROR_DATA : GLUenum =                  100109;
pub const GLU_TESS_EDGE_FLAG_DATA : GLUenum =              100110;
pub const GLU_TESS_COMBINE_DATA : GLUenum =                100111;

/* TessContour */
pub const GLU_CW : GLUenum =                               100120;
pub const GLU_CCW : GLUenum =                              100121;
pub const GLU_INTERIOR : GLUenum =                         100122;
pub const GLU_EXTERIOR : GLUenum =                         100123;
pub const GLU_UNKNOWN : GLUenum =                          100124;

/* TessProperty */
pub const GLU_TESS_WINDING_RULE : GLUenum =                100140;
pub const GLU_TESS_BOUNDARY_ONLY : GLUenum =               100141;
pub const GLU_TESS_TOLERANCE : GLUenum =                   100142;

/* TessError */
pub const GLU_TESS_ERROR1 : GLUenum =                      100151;
pub const GLU_TESS_ERROR2 : GLUenum =                      100152;
pub const GLU_TESS_ERROR3 : GLUenum =                      100153;
pub const GLU_TESS_ERROR4 : GLUenum =                      100154;
pub const GLU_TESS_ERROR5 : GLUenum =                      100155;
pub const GLU_TESS_ERROR6 : GLUenum =                      100156;
pub const GLU_TESS_ERROR7 : GLUenum =                      100157;
pub const GLU_TESS_ERROR8 : GLUenum =                      100158;
pub const GLU_TESS_MISSING_BEGIN_POLYGON : GLUenum =       100151;
pub const GLU_TESS_MISSING_BEGIN_CONTOUR : GLUenum =       100152;
pub const GLU_TESS_MISSING_END_POLYGON : GLUenum =         100153;
pub const GLU_TESS_MISSING_END_CONTOUR : GLUenum =         100154;
pub const GLU_TESS_COORD_TOO_LARGE : GLUenum =             100155;
pub const GLU_TESS_NEED_COMBINE_CALLBACK : GLUenum =       100156;

/* TessWinding */
pub const GLU_TESS_WINDING_ODD : GLUenum =                 100130;
pub const GLU_TESS_WINDING_NONZERO : GLUenum =             100131;
pub const GLU_TESS_WINDING_POSITIVE : GLUenum =            100132;
pub const GLU_TESS_WINDING_NEGATIVE : GLUenum =            100133;
pub const GLU_TESS_WINDING_ABS_GEQ_TWO : GLUenum =         100134;

/*************************************************************/

pub type GLUnurbs = u8;
pub type GLUquadric = u8;
pub type GLUtesselator = u8;

pub type GLUnurbsObj = GLUnurbs;
pub type GLUquadricObj = GLUquadric;
pub type GLUtesselatorObj = GLUtesselator;
pub type GLUtriangulatorObj = GLUtesselator;

pub const GLU_TESS_MAX_COORD : GLUfloat = 1.0e150;

pub extern "C" {

pub fn gluBeginCurve(nurb: *GLUnurbs);
pub fn gluBeginPolygon(tess: *GLUtesselator);
pub fn gluBeginSurface(nurb: *GLUnurbs);
pub fn gluBeginTrim(nurb: *GLUnurbs);
pub fn gluBuild1DMipmapLevels(target: GLenum, internalFormat: GLint, width: GLsizei, format: GLenum, _type: GLenum, level: GLint, base: GLint, max: GLint, data: *voidstar /*const*/) -> GLint ;
pub fn gluBuild1DMipmaps(target: GLenum, internalFormat: GLint, width: GLsizei, format: GLenum, _type: GLenum, data: *voidstar /*const*/) -> GLint ;
pub fn gluBuild2DMipmapLevels(target: GLenum, internalFormat: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, level: GLint, base: GLint, max: GLint, data: *voidstar /*const*/) -> GLint ;
pub fn gluBuild2DMipmaps(target: GLenum, internalFormat: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, data: *voidstar /*const*/) -> GLint ;
pub fn gluBuild3DMipmapLevels(target: GLenum, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, _type: GLenum, level: GLint, base: GLint, max: GLint, data: *voidstar /*const*/) -> GLint ;
pub fn gluBuild3DMipmaps(target: GLenum, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, _type: GLenum, data: *voidstar /*const*/) -> GLint ;
pub fn gluCheckExtension(extName: *GLubyte /*const*/, extString: *GLubyte /*const*/) -> GLboolean ;
pub fn gluCylinder(quad: *GLUquadric, base: GLdouble, top: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);
pub fn gluDeleteNurbsRenderer(nurb: *GLUnurbs);
pub fn gluDeleteQuadric(quad: *GLUquadric);
pub fn gluDeleteTess(tess: *GLUtesselator);
pub fn gluDisk(quad: *GLUquadric, inner: GLdouble, outer: GLdouble, slices: GLint, loops: GLint);
pub fn gluEndCurve(nurb: *GLUnurbs);
pub fn gluEndPolygon(tess: *GLUtesselator);
pub fn gluEndSurface(nurb: *GLUnurbs);
pub fn gluEndTrim(nurb: *GLUnurbs);
pub fn gluErrorString(error: GLenum) -> /*const*/ *GLubyte ;
pub fn gluGetNurbsProperty(nurb: *GLUnurbs, property: GLenum, data: *GLfloat);
pub fn gluGetString(name: GLenum) -> /*const*/ *GLubyte  ;
pub fn gluGetTessProperty(tess: *GLUtesselator, which: GLenum, data: *GLdouble);
pub fn gluLoadSamplingMatrices(nurb: *GLUnurbs, model: *GLfloat /*const*/, perspective: *GLfloat /*const*/, view: *GLint /*const*/);
pub fn gluLookAt(eyeX: GLdouble, eyeY: GLdouble, eyeZ: GLdouble, centerX: GLdouble, centerY: GLdouble, centerZ: GLdouble, upX: GLdouble, upY: GLdouble, upZ: GLdouble);
pub fn gluNewNurbsRenderer() -> *GLUnurbs ;
pub fn gluNewQuadric() -> *GLUquadric ;
pub fn gluNewTess() -> *GLUtesselator ;
pub fn gluNextContour(tess: *GLUtesselator, _type: GLenum);
pub fn gluNurbsCallbackData(nurb: *GLUnurbs, userData: *GLvoid);
pub fn gluNurbsCallbackDataEXT(nurb: *GLUnurbs, userData: *GLvoid);
pub fn gluNurbsCurve(nurb: *GLUnurbs, knotCount: GLint, knots: *GLfloat, stride: GLint, control: *GLfloat, order: GLint, _type: GLenum);
pub fn gluNurbsProperty(nurb: *GLUnurbs, property: GLenum, value: GLfloat);
pub fn gluNurbsSurface(nurb: *GLUnurbs, sKnotCount: GLint, sKnots: *GLfloat, tKnotCount: GLint, tKnots: *GLfloat, sStride: GLint, tStride: GLint, control: *GLfloat, sOrder: GLint, tOrder: GLint, _type: GLenum);
pub fn gluOrtho2D(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble);
pub fn gluPartialDisk(quad: *GLUquadric, inner: GLdouble, outer: GLdouble, slices: GLint, loops: GLint, start: GLdouble, sweep: GLdouble);
pub fn gluPerspective(fovy: GLdouble, aspect: GLdouble, zNear: GLdouble, zFar: GLdouble);
pub fn gluPickMatrix(x: GLdouble, y: GLdouble, delX: GLdouble, delY: GLdouble, viewport: *GLint);
pub fn gluProject(objX: GLdouble, objY: GLdouble, objZ: GLdouble, model: *GLdouble /*const*/, proj: *GLdouble /*const*/, view: *GLint /*const*/, winX: *GLdouble, winY: *GLdouble, winZ: *GLdouble) -> GLint ;
pub fn gluPwlCurve(nurb: *GLUnurbs, count: GLint, data: *GLfloat, stride: GLint, _type: GLenum);
pub fn gluQuadricCallback(quad: *GLUquadric, which: GLenum, CallBackFunc: nullary_void_callback);
pub fn gluQuadricDrawStyle(quad: *GLUquadric, draw: GLenum);
pub fn gluQuadricNormals(quad: *GLUquadric, normal: GLenum);
pub fn gluQuadricOrientation(quad: *GLUquadric, orientation: GLenum);
pub fn gluQuadricTexture(quad: *GLUquadric, texture: GLboolean);
pub fn gluScaleImage(format: GLenum, wIn: GLsizei, hIn: GLsizei, typeIn: GLenum, dataIn: *voidstar /*const*/, wOut: GLsizei, hOut: GLsizei, typeOut: GLenum, dataOut: *GLvoid) -> GLint ;
pub fn gluSphere(quad: *GLUquadric, radius: GLdouble, slices: GLint, stacks: GLint);
pub fn gluTessBeginContour(tess: *GLUtesselator);
pub fn gluTessBeginPolygon(tess: *GLUtesselator, data: *GLvoid);
pub fn gluTessCallback(tess: *GLUtesselator, which: GLenum, CallBackFunc: nullary_void_callback);
pub fn gluTessEndContour(tess: *GLUtesselator);
pub fn gluTessEndPolygon(tess: *GLUtesselator);
pub fn gluTessNormal(tess: *GLUtesselator, valueX: GLdouble, valueY: GLdouble, valueZ: GLdouble);
pub fn gluTessProperty(tess: *GLUtesselator, which: GLenum, data: GLdouble);
pub fn gluTessVertex(tess: *GLUtesselator, location: *GLdouble, data: *GLvoid);
pub fn gluUnProject(winX: GLdouble, winY: GLdouble, winZ: GLdouble, model: *GLdouble /*const*/, proj: *GLdouble /*const*/, view: *GLint /*const*/, objX: *GLdouble, objY: *GLdouble, objZ: *GLdouble) -> GLint ;
pub fn gluUnProject4(winX: GLdouble, winY: GLdouble, winZ: GLdouble, clipW: GLdouble, model: *GLdouble /*const*/, proj: *GLdouble /*const*/, view: *GLint /*const*/, nearPlane: GLdouble, farPlane: GLdouble, objX: *GLdouble, objY: *GLdouble, objZ: *GLdouble, objW: *GLdouble) -> GLint ;

}