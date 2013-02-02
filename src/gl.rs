use core::libc::*;

pub type GLenum = c_uint;

pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLbyte = c_schar;
pub type GLshort = c_short;
pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLubyte = c_uchar;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLvoid = c_void;

pub type GLintptr = c_long;
pub type GLsizeiptr = c_long;


pub type GLchar = c_char;

/* Version */
pub const VERSION_MAJOR : c_int =  2;
pub const VERSION_MINOR : c_int =  1;

/* AccumOp */
pub const GL_ACCUM : GLenum =                         0x0100;
pub const GL_LOAD : GLenum =                          0x0101;
pub const GL_RETURN : GLenum =                        0x0102;
pub const GL_MULT : GLenum =                          0x0103;
pub const GL_ADD : GLenum =                           0x0104;

/* AlphaFunction */
pub const GL_NEVER : GLenum =                         0x0200;
pub const GL_LESS : GLenum =                          0x0201;
pub const GL_EQUAL : GLenum =                         0x0202;
pub const GL_LEQUAL : GLenum =                        0x0203;
pub const GL_GREATER : GLenum =                       0x0204;
pub const GL_NOTEQUAL : GLenum =                      0x0205;
pub const GL_GEQUAL : GLenum =                        0x0206;
pub const GL_ALWAYS : GLenum =                        0x0207;

/* AttribMask */
pub const GL_CURRENT_BIT : GLenum =                   0x00000001;
pub const GL_POINT_BIT : GLenum =                     0x00000002;
pub const GL_LINE_BIT : GLenum =                      0x00000004;
pub const GL_POLYGON_BIT : GLenum =                   0x00000008;
pub const GL_POLYGON_STIPPLE_BIT : GLenum =           0x00000010;
pub const GL_PIXEL_MODE_BIT : GLenum =                0x00000020;
pub const GL_LIGHTING_BIT : GLenum =                  0x00000040;
pub const GL_FOG_BIT : GLenum =                       0x00000080;
pub const GL_DEPTH_BUFFER_BIT : GLenum =              0x00000100;
pub const GL_ACCUM_BUFFER_BIT : GLenum =              0x00000200;
pub const GL_STENCIL_BUFFER_BIT : GLenum =            0x00000400;
pub const GL_VIEWPORT_BIT : GLenum =                  0x00000800;
pub const GL_TRANSFORM_BIT : GLenum =                 0x00001000;
pub const GL_ENABLE_BIT : GLenum =                    0x00002000;
pub const GL_COLOR_BUFFER_BIT : GLenum =              0x00004000;
pub const GL_HINT_BIT : GLenum =                      0x00008000;
pub const GL_EVAL_BIT : GLenum =                      0x00010000;
pub const GL_LIST_BIT : GLenum =                      0x00020000;
pub const GL_TEXTURE_BIT : GLenum =                   0x00040000;
pub const GL_SCISSOR_BIT : GLenum =                   0x00080000;
pub const GL_ALL_ATTRIB_BITS : GLenum =               0x000fffff;

/* BeginMode */
pub const GL_POINTS : GLenum =                        0x0000;
pub const GL_LINES : GLenum =                         0x0001;
pub const GL_LINE_LOOP : GLenum =                     0x0002;
pub const GL_LINE_STRIP : GLenum =                    0x0003;
pub const GL_TRIANGLES : GLenum =                     0x0004;
pub const GL_TRIANGLE_STRIP : GLenum =                0x0005;
pub const GL_TRIANGLE_FAN : GLenum =                  0x0006;
pub const GL_QUADS : GLenum =                         0x0007;
pub const GL_QUAD_STRIP : GLenum =                    0x0008;
pub const GL_POLYGON : GLenum =                       0x0009;

/* BlendEquationMode */
/*      GL_LOGIC_OP */
/*      GL_FUNC_ADD */
/*      GL_MIN */
/*      GL_MAX */
/*      GL_FUNC_SUBTRACT */
/*      GL_FUNC_REVERSE_SUBTRACT */

/* BlendingFactorDest */
pub const GL_ZERO : GLenum =                          0;
pub const GL_ONE : GLenum =                           1;
pub const GL_SRC_COLOR : GLenum =                     0x0300;
pub const GL_ONE_MINUS_SRC_COLOR : GLenum =           0x0301;
pub const GL_SRC_ALPHA : GLenum =                     0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA : GLenum =           0x0303;
pub const GL_DST_ALPHA : GLenum =                     0x0304;
pub const GL_ONE_MINUS_DST_ALPHA : GLenum =           0x0305;
/*      GL_CONSTANT_COLOR */
/*      GL_ONE_MINUS_CONSTANT_COLOR */
/*      GL_CONSTANT_ALPHA */
/*      GL_ONE_MINUS_CONSTANT_ALPHA */

/* BlendingFactorSrc */
/*      GL_ZERO */
/*      GL_ONE */
pub const GL_DST_COLOR : GLenum =                     0x0306;
pub const GL_ONE_MINUS_DST_COLOR : GLenum =           0x0307;
pub const GL_SRC_ALPHA_SATURATE : GLenum =            0x0308;
/*      GL_SRC_ALPHA */
/*      GL_ONE_MINUS_SRC_ALPHA */
/*      GL_DST_ALPHA */
/*      GL_ONE_MINUS_DST_ALPHA */
/*      GL_CONSTANT_COLOR */
/*      GL_ONE_MINUS_CONSTANT_COLOR */
/*      GL_CONSTANT_ALPHA */
/*      GL_ONE_MINUS_CONSTANT_ALPHA */

/* Boolean */
pub const GL_TRUE : GLenum =                          1;
pub const GL_FALSE : GLenum =                         0;

/* ClearBufferMask */
/*      GL_COLOR_BUFFER_BIT */
/*      GL_ACCUM_BUFFER_BIT */
/*      GL_STENCIL_BUFFER_BIT */
/*      GL_DEPTH_BUFFER_BIT */

/* ClientArrayType */
/*      GL_VERTEX_ARRAY */
/*      GL_NORMAL_ARRAY */
/*      GL_COLOR_ARRAY */
/*      GL_INDEX_ARRAY */
/*      GL_TEXTURE_COORD_ARRAY */
/*      GL_EDGE_FLAG_ARRAY */

/* ClipPlaneName */
pub const GL_CLIP_PLANE0 : GLenum =                   0x3000;
pub const GL_CLIP_PLANE1 : GLenum =                   0x3001;
pub const GL_CLIP_PLANE2 : GLenum =                   0x3002;
pub const GL_CLIP_PLANE3 : GLenum =                   0x3003;
pub const GL_CLIP_PLANE4 : GLenum =                   0x3004;
pub const GL_CLIP_PLANE5 : GLenum =                   0x3005;

/* ColorMaterialFace */
/*      GL_FRONT */
/*      GL_BACK */
/*      GL_FRONT_AND_BACK */

/* ColorMaterialParameter */
/*      GL_AMBIENT */
/*      GL_DIFFUSE */
/*      GL_SPECULAR */
/*      GL_EMISSION */
/*      GL_AMBIENT_AND_DIFFUSE */

/* ColorPointerType */
/*      GL_BYTE */
/*      GL_UNSIGNED_BYTE */
/*      GL_SHORT */
/*      GL_UNSIGNED_SHORT */
/*      GL_INT */
/*      GL_UNSIGNED_INT */
/*      GL_FLOAT */
/*      GL_DOUBLE */

/* ColorTableParameterPName */
/*      GL_COLOR_TABLE_SCALE */
/*      GL_COLOR_TABLE_BIAS */

/* ColorTableTarget */
/*      GL_COLOR_TABLE */
/*      GL_POST_CONVOLUTION_COLOR_TABLE */
/*      GL_POST_COLOR_MATRIX_COLOR_TABLE */
/*      GL_PROXY_COLOR_TABLE */
/*      GL_PROXY_POST_CONVOLUTION_COLOR_TABLE */
/*      GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE */

/* ConvolutionBorderMode */
/*      GL_REDUCE */
/*      GL_IGNORE_BORDER */
/*      GL_CONSTANT_BORDER */

/* ConvolutionParameter */
/*      GL_CONVOLUTION_BORDER_MODE */
/*      GL_CONVOLUTION_FILTER_SCALE */
/*      GL_CONVOLUTION_FILTER_BIAS */

/* ConvolutionTarget */
/*      GL_CONVOLUTION_1D */
/*      GL_CONVOLUTION_2D */

/* CullFaceMode */
/*      GL_FRONT */
/*      GL_BACK */
/*      GL_FRONT_AND_BACK */

/* DataType */
pub const GL_BYTE : GLenum =                          0x1400;
pub const GL_UNSIGNED_BYTE : GLenum =                 0x1401;
pub const GL_SHORT : GLenum =                         0x1402;
pub const GL_UNSIGNED_SHORT : GLenum =                0x1403;
pub const GL_INT : GLenum =                           0x1404;
pub const GL_UNSIGNED_INT : GLenum =                  0x1405;
pub const GL_FLOAT : GLenum =                         0x1406;
pub const GL_2_BYTES : GLenum =                       0x1407;
pub const GL_3_BYTES : GLenum =                       0x1408;
pub const GL_4_BYTES : GLenum =                       0x1409;
pub const GL_DOUBLE : GLenum =                        0x140A;

/* DepthFunction */
/*      GL_NEVER */
/*      GL_LESS */
/*      GL_EQUAL */
/*      GL_LEQUAL */
/*      GL_GREATER */
/*      GL_NOTEQUAL */
/*      GL_GEQUAL */
/*      GL_ALWAYS */

/* DrawBufferMode */
pub const GL_NONE : GLenum =                          0;
pub const GL_FRONT_LEFT : GLenum =                    0x0400;
pub const GL_FRONT_RIGHT : GLenum =                   0x0401;
pub const GL_BACK_LEFT : GLenum =                     0x0402;
pub const GL_BACK_RIGHT : GLenum =                    0x0403;
pub const GL_FRONT : GLenum =                         0x0404;
pub const GL_BACK : GLenum =                          0x0405;
pub const GL_LEFT : GLenum =                          0x0406;
pub const GL_RIGHT : GLenum =                         0x0407;
pub const GL_FRONT_AND_BACK : GLenum =                0x0408;
pub const GL_AUX0 : GLenum =                          0x0409;
pub const GL_AUX1 : GLenum =                          0x040A;
pub const GL_AUX2 : GLenum =                          0x040B;
pub const GL_AUX3 : GLenum =                          0x040C;

/* Enable */
/*      GL_FOG */
/*      GL_LIGHTING */
/*      GL_TEXTURE_1D */
/*      GL_TEXTURE_2D */
/*      GL_LINE_STIPPLE */
/*      GL_POLYGON_STIPPLE */
/*      GL_CULL_FACE */
/*      GL_ALPHA_TEST */
/*      GL_BLEND */
/*      GL_INDEX_LOGIC_OP */
/*      GL_COLOR_LOGIC_OP */
/*      GL_DITHER */
/*      GL_STENCIL_TEST */
/*      GL_DEPTH_TEST */
/*      GL_CLIP_PLANE0 */
/*      GL_CLIP_PLANE1 */
/*      GL_CLIP_PLANE2 */
/*      GL_CLIP_PLANE3 */
/*      GL_CLIP_PLANE4 */
/*      GL_CLIP_PLANE5 */
/*      GL_LIGHT0 */
/*      GL_LIGHT1 */
/*      GL_LIGHT2 */
/*      GL_LIGHT3 */
/*      GL_LIGHT4 */
/*      GL_LIGHT5 */
/*      GL_LIGHT6 */
/*      GL_LIGHT7 */
/*      GL_TEXTURE_GEN_S */
/*      GL_TEXTURE_GEN_T */
/*      GL_TEXTURE_GEN_R */
/*      GL_TEXTURE_GEN_Q */
/*      GL_MAP1_VERTEX_3 */
/*      GL_MAP1_VERTEX_4 */
/*      GL_MAP1_COLOR_4 */
/*      GL_MAP1_INDEX */
/*      GL_MAP1_NORMAL */
/*      GL_MAP1_TEXTURE_COORD_1 */
/*      GL_MAP1_TEXTURE_COORD_2 */
/*      GL_MAP1_TEXTURE_COORD_3 */
/*      GL_MAP1_TEXTURE_COORD_4 */
/*      GL_MAP2_VERTEX_3 */
/*      GL_MAP2_VERTEX_4 */
/*      GL_MAP2_COLOR_4 */
/*      GL_MAP2_INDEX */
/*      GL_MAP2_NORMAL */
/*      GL_MAP2_TEXTURE_COORD_1 */
/*      GL_MAP2_TEXTURE_COORD_2 */
/*      GL_MAP2_TEXTURE_COORD_3 */
/*      GL_MAP2_TEXTURE_COORD_4 */
/*      GL_POINT_SMOOTH */
/*      GL_LINE_SMOOTH */
/*      GL_POLYGON_SMOOTH */
/*      GL_SCISSOR_TEST */
/*      GL_COLOR_MATERIAL */
/*      GL_NORMALIZE */
/*      GL_AUTO_NORMAL */
/*      GL_VERTEX_ARRAY */
/*      GL_NORMAL_ARRAY */
/*      GL_COLOR_ARRAY */
/*      GL_INDEX_ARRAY */
/*      GL_TEXTURE_COORD_ARRAY */
/*      GL_EDGE_FLAG_ARRAY */
/*      GL_POLYGON_OFFSET_POINT */
/*      GL_POLYGON_OFFSET_LINE */
/*      GL_POLYGON_OFFSET_FILL */
/*      GL_COLOR_TABLE */
/*      GL_POST_CONVOLUTION_COLOR_TABLE */
/*      GL_POST_COLOR_MATRIX_COLOR_TABLE */
/*      GL_CONVOLUTION_1D */
/*      GL_CONVOLUTION_2D */
/*      GL_SEPARABLE_2D */
/*      GL_HISTOGRAM */
/*      GL_MINMAX */
/*      GL_RESCALE_NORMAL */
/*      GL_TEXTURE_3D */

/* ErrorCode */
pub const GL_NO_ERROR : GLenum =                      0;
pub const GL_INVALID_ENUM : GLenum =                  0x0500;
pub const GL_INVALID_VALUE : GLenum =                 0x0501;
pub const GL_INVALID_OPERATION : GLenum =             0x0502;
pub const GL_STACK_OVERFLOW : GLenum =                0x0503;
pub const GL_STACK_UNDERFLOW : GLenum =               0x0504;
pub const GL_OUT_OF_MEMORY : GLenum =                 0x0505;
/*      GL_TABLE_TOO_LARGE */

/* FeedBackMode */
pub const GL_2D : GLenum =                            0x0600;
pub const GL_3D : GLenum =                            0x0601;
pub const GL_3D_COLOR : GLenum =                      0x0602;
pub const GL_3D_COLOR_TEXTURE : GLenum =              0x0603;
pub const GL_4D_COLOR_TEXTURE : GLenum =              0x0604;

/* FeedBackToken */
pub const GL_PASS_THROUGH_TOKEN : GLenum =            0x0700;
pub const GL_POINT_TOKEN : GLenum =                   0x0701;
pub const GL_LINE_TOKEN : GLenum =                    0x0702;
pub const GL_POLYGON_TOKEN : GLenum =                 0x0703;
pub const GL_BITMAP_TOKEN : GLenum =                  0x0704;
pub const GL_DRAW_PIXEL_TOKEN : GLenum =              0x0705;
pub const GL_COPY_PIXEL_TOKEN : GLenum =              0x0706;
pub const GL_LINE_RESET_TOKEN : GLenum =              0x0707;

/* FogMode */
/*      GL_LINEAR */
pub const GL_EXP : GLenum =                           0x0800;
pub const GL_EXP2 : GLenum =                          0x0801;

/* FogParameter */
/*      GL_FOG_COLOR */
/*      GL_FOG_DENSITY */
/*      GL_FOG_END */
/*      GL_FOG_INDEX */
/*      GL_FOG_MODE */
/*      GL_FOG_START */

/* FrontFaceDirection */
pub const GL_CW : GLenum =                            0x0900;
pub const GL_CCW : GLenum =                           0x0901;

/* GetColorTableParameterPName */
/*      GL_COLOR_TABLE_SCALE */
/*      GL_COLOR_TABLE_BIAS */
/*      GL_COLOR_TABLE_FORMAT */
/*      GL_COLOR_TABLE_WIDTH */
/*      GL_COLOR_TABLE_RED_SIZE */
/*      GL_COLOR_TABLE_GREEN_SIZE */
/*      GL_COLOR_TABLE_BLUE_SIZE */
/*      GL_COLOR_TABLE_ALPHA_SIZE */
/*      GL_COLOR_TABLE_LUMINANCE_SIZE */
/*      GL_COLOR_TABLE_INTENSITY_SIZE */

/* GetConvolutionParameterPName */
/*      GL_CONVOLUTION_BORDER_COLOR */
/*      GL_CONVOLUTION_BORDER_MODE */
/*      GL_CONVOLUTION_FILTER_SCALE */
/*      GL_CONVOLUTION_FILTER_BIAS */
/*      GL_CONVOLUTION_FORMAT */
/*      GL_CONVOLUTION_WIDTH */
/*      GL_CONVOLUTION_HEIGHT */
/*      GL_MAX_CONVOLUTION_WIDTH */
/*      GL_MAX_CONVOLUTION_HEIGHT */

/* GetHistogramParameterPName */
/*      GL_HISTOGRAM_WIDTH */
/*      GL_HISTOGRAM_FORMAT */
/*      GL_HISTOGRAM_RED_SIZE */
/*      GL_HISTOGRAM_GREEN_SIZE */
/*      GL_HISTOGRAM_BLUE_SIZE */
/*      GL_HISTOGRAM_ALPHA_SIZE */
/*      GL_HISTOGRAM_LUMINANCE_SIZE */
/*      GL_HISTOGRAM_SINK */

/* GetMapTarget */
pub const GL_COEFF : GLenum =                         0x0A00;
pub const GL_ORDER : GLenum =                         0x0A01;
pub const GL_DOMAIN : GLenum =                        0x0A02;

/* GetMinmaxParameterPName */
/*      GL_MINMAX_FORMAT */
/*      GL_MINMAX_SINK */

/* GetPixelMap */
/*      GL_PIXEL_MAP_I_TO_I */
/*      GL_PIXEL_MAP_S_TO_S */
/*      GL_PIXEL_MAP_I_TO_R */
/*      GL_PIXEL_MAP_I_TO_G */
/*      GL_PIXEL_MAP_I_TO_B */
/*      GL_PIXEL_MAP_I_TO_A */
/*      GL_PIXEL_MAP_R_TO_R */
/*      GL_PIXEL_MAP_G_TO_G */
/*      GL_PIXEL_MAP_B_TO_B */
/*      GL_PIXEL_MAP_A_TO_A */

/* GetPointerTarget */
/*      GL_VERTEX_ARRAY_POINTER */
/*      GL_NORMAL_ARRAY_POINTER */
/*      GL_COLOR_ARRAY_POINTER */
/*      GL_INDEX_ARRAY_POINTER */
/*      GL_TEXTURE_COORD_ARRAY_POINTER */
/*      GL_EDGE_FLAG_ARRAY_POINTER */

/* GetTarget */
pub const GL_CURRENT_COLOR : GLenum =                 0x0B00;
pub const GL_CURRENT_INDEX : GLenum =                 0x0B01;
pub const GL_CURRENT_NORMAL : GLenum =                0x0B02;
pub const GL_CURRENT_TEXTURE_COORDS : GLenum =        0x0B03;
pub const GL_CURRENT_RASTER_COLOR : GLenum =          0x0B04;
pub const GL_CURRENT_RASTER_INDEX : GLenum =          0x0B05;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS : GLenum = 0x0B06;
pub const GL_CURRENT_RASTER_POSITION : GLenum =       0x0B07;
pub const GL_CURRENT_RASTER_POSITION_VALID : GLenum = 0x0B08;
pub const GL_CURRENT_RASTER_DISTANCE : GLenum =       0x0B09;
pub const GL_POINT_SMOOTH : GLenum =                  0x0B10;
pub const GL_POINT_SIZE : GLenum =                    0x0B11;
pub const GL_POINT_SIZE_RANGE : GLenum =              0x0B12;
pub const GL_POINT_SIZE_GRANULARITY : GLenum =        0x0B13;
pub const GL_LINE_SMOOTH : GLenum =                   0x0B20;
pub const GL_LINE_WIDTH : GLenum =                    0x0B21;
pub const GL_LINE_WIDTH_RANGE : GLenum =              0x0B22;
pub const GL_LINE_WIDTH_GRANULARITY : GLenum =        0x0B23;
pub const GL_LINE_STIPPLE : GLenum =                  0x0B24;
pub const GL_LINE_STIPPLE_PATTERN : GLenum =          0x0B25;
pub const GL_LINE_STIPPLE_REPEAT : GLenum =           0x0B26;
/*      GL_SMOOTH_POINT_SIZE_RANGE */
/*      GL_SMOOTH_POINT_SIZE_GRANULARITY */
/*      GL_SMOOTH_LINE_WIDTH_RANGE */
/*      GL_SMOOTH_LINE_WIDTH_GRANULARITY */
/*      GL_ALIASED_POINT_SIZE_RANGE */
/*      GL_ALIASED_LINE_WIDTH_RANGE */
pub const GL_LIST_MODE : GLenum =                     0x0B30;
pub const GL_MAX_LIST_NESTING : GLenum =              0x0B31;
pub const GL_LIST_BASE : GLenum =                     0x0B32;
pub const GL_LIST_INDEX : GLenum =                    0x0B33;
pub const GL_POLYGON_MODE : GLenum =                  0x0B40;
pub const GL_POLYGON_SMOOTH : GLenum =                0x0B41;
pub const GL_POLYGON_STIPPLE : GLenum =               0x0B42;
pub const GL_EDGE_FLAG : GLenum =                     0x0B43;
pub const GL_CULL_FACE : GLenum =                     0x0B44;
pub const GL_CULL_FACE_MODE : GLenum =                0x0B45;
pub const GL_FRONT_FACE : GLenum =                    0x0B46;
pub const GL_LIGHTING : GLenum =                      0x0B50;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER : GLenum =      0x0B51;
pub const GL_LIGHT_MODEL_TWO_SIDE : GLenum =          0x0B52;
pub const GL_LIGHT_MODEL_AMBIENT : GLenum =           0x0B53;
pub const GL_SHADE_MODEL : GLenum =                   0x0B54;
pub const GL_COLOR_MATERIAL_FACE : GLenum =           0x0B55;
pub const GL_COLOR_MATERIAL_PARAMETER : GLenum =      0x0B56;
pub const GL_COLOR_MATERIAL : GLenum =                0x0B57;
pub const GL_FOG : GLenum =                           0x0B60;
pub const GL_FOG_INDEX : GLenum =                     0x0B61;
pub const GL_FOG_DENSITY : GLenum =                   0x0B62;
pub const GL_FOG_START : GLenum =                     0x0B63;
pub const GL_FOG_END : GLenum =                       0x0B64;
pub const GL_FOG_MODE : GLenum =                      0x0B65;
pub const GL_FOG_COLOR : GLenum =                     0x0B66;
pub const GL_DEPTH_RANGE : GLenum =                   0x0B70;
pub const GL_DEPTH_TEST : GLenum =                    0x0B71;
pub const GL_DEPTH_WRITEMASK : GLenum =               0x0B72;
pub const GL_DEPTH_CLEAR_VALUE : GLenum =             0x0B73;
pub const GL_DEPTH_FUNC : GLenum =                    0x0B74;
pub const GL_ACCUM_CLEAR_VALUE : GLenum =             0x0B80;
pub const GL_STENCIL_TEST : GLenum =                  0x0B90;
pub const GL_STENCIL_CLEAR_VALUE : GLenum =           0x0B91;
pub const GL_STENCIL_FUNC : GLenum =                  0x0B92;
pub const GL_STENCIL_VALUE_MASK : GLenum =            0x0B93;
pub const GL_STENCIL_FAIL : GLenum =                  0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL : GLenum =       0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS : GLenum =       0x0B96;
pub const GL_STENCIL_REF : GLenum =                   0x0B97;
pub const GL_STENCIL_WRITEMASK : GLenum =             0x0B98;
pub const GL_MATRIX_MODE : GLenum =                   0x0BA0;
pub const GL_NORMALIZE : GLenum =                     0x0BA1;
pub const GL_VIEWPORT : GLenum =                      0x0BA2;
pub const GL_MODELVIEW_STACK_DEPTH : GLenum =         0x0BA3;
pub const GL_PROJECTION_STACK_DEPTH : GLenum =        0x0BA4;
pub const GL_TEXTURE_STACK_DEPTH : GLenum =           0x0BA5;
pub const GL_MODELVIEW_MATRIX : GLenum =              0x0BA6;
pub const GL_PROJECTION_MATRIX : GLenum =             0x0BA7;
pub const GL_TEXTURE_MATRIX : GLenum =                0x0BA8;
pub const GL_ATTRIB_STACK_DEPTH : GLenum =            0x0BB0;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH : GLenum =     0x0BB1;
pub const GL_ALPHA_TEST : GLenum =                    0x0BC0;
pub const GL_ALPHA_TEST_FUNC : GLenum =               0x0BC1;
pub const GL_ALPHA_TEST_REF : GLenum =                0x0BC2;
pub const GL_DITHER : GLenum =                        0x0BD0;
pub const GL_BLEND_DST : GLenum =                     0x0BE0;
pub const GL_BLEND_SRC : GLenum =                     0x0BE1;
pub const GL_BLEND : GLenum =                         0x0BE2;
pub const GL_LOGIC_OP_MODE : GLenum =                 0x0BF0;
pub const GL_INDEX_LOGIC_OP : GLenum =                0x0BF1;
pub const GL_COLOR_LOGIC_OP : GLenum =                0x0BF2;
pub const GL_AUX_BUFFERS : GLenum =                   0x0C00;
pub const GL_DRAW_BUFFER : GLenum =                   0x0C01;
pub const GL_READ_BUFFER : GLenum =                   0x0C02;
pub const GL_SCISSOR_BOX : GLenum =                   0x0C10;
pub const GL_SCISSOR_TEST : GLenum =                  0x0C11;
pub const GL_INDEX_CLEAR_VALUE : GLenum =             0x0C20;
pub const GL_INDEX_WRITEMASK : GLenum =               0x0C21;
pub const GL_COLOR_CLEAR_VALUE : GLenum =             0x0C22;
pub const GL_COLOR_WRITEMASK : GLenum =               0x0C23;
pub const GL_INDEX_MODE : GLenum =                    0x0C30;
pub const GL_RGBA_MODE : GLenum =                     0x0C31;
pub const GL_DOUBLEBUFFER : GLenum =                  0x0C32;
pub const GL_STEREO : GLenum =                        0x0C33;
pub const GL_RENDER_MODE : GLenum =                   0x0C40;
pub const GL_PERSPECTIVE_CORRECTION_HINT : GLenum =   0x0C50;
pub const GL_POINT_SMOOTH_HINT : GLenum =             0x0C51;
pub const GL_LINE_SMOOTH_HINT : GLenum =              0x0C52;
pub const GL_POLYGON_SMOOTH_HINT : GLenum =           0x0C53;
pub const GL_FOG_HINT : GLenum =                      0x0C54;
pub const GL_TEXTURE_GEN_S : GLenum =                 0x0C60;
pub const GL_TEXTURE_GEN_T : GLenum =                 0x0C61;
pub const GL_TEXTURE_GEN_R : GLenum =                 0x0C62;
pub const GL_TEXTURE_GEN_Q : GLenum =                 0x0C63;
pub const GL_PIXEL_MAP_I_TO_I : GLenum =              0x0C70;
pub const GL_PIXEL_MAP_S_TO_S : GLenum =              0x0C71;
pub const GL_PIXEL_MAP_I_TO_R : GLenum =              0x0C72;
pub const GL_PIXEL_MAP_I_TO_G : GLenum =              0x0C73;
pub const GL_PIXEL_MAP_I_TO_B : GLenum =              0x0C74;
pub const GL_PIXEL_MAP_I_TO_A : GLenum =              0x0C75;
pub const GL_PIXEL_MAP_R_TO_R : GLenum =              0x0C76;
pub const GL_PIXEL_MAP_G_TO_G : GLenum =              0x0C77;
pub const GL_PIXEL_MAP_B_TO_B : GLenum =              0x0C78;
pub const GL_PIXEL_MAP_A_TO_A : GLenum =              0x0C79;
pub const GL_PIXEL_MAP_I_TO_I_SIZE : GLenum =         0x0CB0;
pub const GL_PIXEL_MAP_S_TO_S_SIZE : GLenum =         0x0CB1;
pub const GL_PIXEL_MAP_I_TO_R_SIZE : GLenum =         0x0CB2;
pub const GL_PIXEL_MAP_I_TO_G_SIZE : GLenum =         0x0CB3;
pub const GL_PIXEL_MAP_I_TO_B_SIZE : GLenum =         0x0CB4;
pub const GL_PIXEL_MAP_I_TO_A_SIZE : GLenum =         0x0CB5;
pub const GL_PIXEL_MAP_R_TO_R_SIZE : GLenum =         0x0CB6;
pub const GL_PIXEL_MAP_G_TO_G_SIZE : GLenum =         0x0CB7;
pub const GL_PIXEL_MAP_B_TO_B_SIZE : GLenum =         0x0CB8;
pub const GL_PIXEL_MAP_A_TO_A_SIZE : GLenum =         0x0CB9;
pub const GL_UNPACK_SWAP_BYTES : GLenum =             0x0CF0;
pub const GL_UNPACK_LSB_FIRST : GLenum =              0x0CF1;
pub const GL_UNPACK_ROW_LENGTH : GLenum =             0x0CF2;
pub const GL_UNPACK_SKIP_ROWS : GLenum =              0x0CF3;
pub const GL_UNPACK_SKIP_PIXELS : GLenum =            0x0CF4;
pub const GL_UNPACK_ALIGNMENT : GLenum =              0x0CF5;
pub const GL_PACK_SWAP_BYTES : GLenum =               0x0D00;
pub const GL_PACK_LSB_FIRST : GLenum =                0x0D01;
pub const GL_PACK_ROW_LENGTH : GLenum =               0x0D02;
pub const GL_PACK_SKIP_ROWS : GLenum =                0x0D03;
pub const GL_PACK_SKIP_PIXELS : GLenum =              0x0D04;
pub const GL_PACK_ALIGNMENT : GLenum =                0x0D05;
pub const GL_MAP_COLOR : GLenum =                     0x0D10;
pub const GL_MAP_STENCIL : GLenum =                   0x0D11;
pub const GL_INDEX_SHIFT : GLenum =                   0x0D12;
pub const GL_INDEX_OFFSET : GLenum =                  0x0D13;
pub const GL_RED_SCALE : GLenum =                     0x0D14;
pub const GL_RED_BIAS : GLenum =                      0x0D15;
pub const GL_ZOOM_X : GLenum =                        0x0D16;
pub const GL_ZOOM_Y : GLenum =                        0x0D17;
pub const GL_GREEN_SCALE : GLenum =                   0x0D18;
pub const GL_GREEN_BIAS : GLenum =                    0x0D19;
pub const GL_BLUE_SCALE : GLenum =                    0x0D1A;
pub const GL_BLUE_BIAS : GLenum =                     0x0D1B;
pub const GL_ALPHA_SCALE : GLenum =                   0x0D1C;
pub const GL_ALPHA_BIAS : GLenum =                    0x0D1D;
pub const GL_DEPTH_SCALE : GLenum =                   0x0D1E;
pub const GL_DEPTH_BIAS : GLenum =                    0x0D1F;
pub const GL_MAX_EVAL_ORDER : GLenum =                0x0D30;
pub const GL_MAX_LIGHTS : GLenum =                    0x0D31;
pub const GL_MAX_CLIP_PLANES : GLenum =               0x0D32;
pub const GL_MAX_TEXTURE_SIZE : GLenum =              0x0D33;
pub const GL_MAX_PIXEL_MAP_TABLE : GLenum =           0x0D34;
pub const GL_MAX_ATTRIB_STACK_DEPTH : GLenum =        0x0D35;
pub const GL_MAX_MODELVIEW_STACK_DEPTH : GLenum =     0x0D36;
pub const GL_MAX_NAME_STACK_DEPTH : GLenum =          0x0D37;
pub const GL_MAX_PROJECTION_STACK_DEPTH : GLenum =    0x0D38;
pub const GL_MAX_TEXTURE_STACK_DEPTH : GLenum =       0x0D39;
pub const GL_MAX_VIEWPORT_DIMS : GLenum =             0x0D3A;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH : GLenum = 0x0D3B;
pub const GL_SUBPIXEL_BITS : GLenum =                 0x0D50;
pub const GL_INDEX_BITS : GLenum =                    0x0D51;
pub const GL_RED_BITS : GLenum =                      0x0D52;
pub const GL_GREEN_BITS : GLenum =                    0x0D53;
pub const GL_BLUE_BITS : GLenum =                     0x0D54;
pub const GL_ALPHA_BITS : GLenum =                    0x0D55;
pub const GL_DEPTH_BITS : GLenum =                    0x0D56;
pub const GL_STENCIL_BITS : GLenum =                  0x0D57;
pub const GL_ACCUM_RED_BITS : GLenum =                0x0D58;
pub const GL_ACCUM_GREEN_BITS : GLenum =              0x0D59;
pub const GL_ACCUM_BLUE_BITS : GLenum =               0x0D5A;
pub const GL_ACCUM_ALPHA_BITS : GLenum =              0x0D5B;
pub const GL_NAME_STACK_DEPTH : GLenum =              0x0D70;
pub const GL_AUTO_NORMAL : GLenum =                   0x0D80;
pub const GL_MAP1_COLOR_4 : GLenum =                  0x0D90;
pub const GL_MAP1_INDEX : GLenum =                    0x0D91;
pub const GL_MAP1_NORMAL : GLenum =                   0x0D92;
pub const GL_MAP1_TEXTURE_COORD_1 : GLenum =          0x0D93;
pub const GL_MAP1_TEXTURE_COORD_2 : GLenum =          0x0D94;
pub const GL_MAP1_TEXTURE_COORD_3 : GLenum =          0x0D95;
pub const GL_MAP1_TEXTURE_COORD_4 : GLenum =          0x0D96;
pub const GL_MAP1_VERTEX_3 : GLenum =                 0x0D97;
pub const GL_MAP1_VERTEX_4 : GLenum =                 0x0D98;
pub const GL_MAP2_COLOR_4 : GLenum =                  0x0DB0;
pub const GL_MAP2_INDEX : GLenum =                    0x0DB1;
pub const GL_MAP2_NORMAL : GLenum =                   0x0DB2;
pub const GL_MAP2_TEXTURE_COORD_1 : GLenum =          0x0DB3;
pub const GL_MAP2_TEXTURE_COORD_2 : GLenum =          0x0DB4;
pub const GL_MAP2_TEXTURE_COORD_3 : GLenum =          0x0DB5;
pub const GL_MAP2_TEXTURE_COORD_4 : GLenum =          0x0DB6;
pub const GL_MAP2_VERTEX_3 : GLenum =                 0x0DB7;
pub const GL_MAP2_VERTEX_4 : GLenum =                 0x0DB8;
pub const GL_MAP1_GRID_DOMAIN : GLenum =              0x0DD0;
pub const GL_MAP1_GRID_SEGMENTS : GLenum =            0x0DD1;
pub const GL_MAP2_GRID_DOMAIN : GLenum =              0x0DD2;
pub const GL_MAP2_GRID_SEGMENTS : GLenum =            0x0DD3;
pub const GL_TEXTURE_1D : GLenum =                    0x0DE0;
pub const GL_TEXTURE_2D : GLenum =                    0x0DE1;
pub const GL_FEEDBACK_BUFFER_POINTER : GLenum =       0x0DF0;
pub const GL_FEEDBACK_BUFFER_SIZE : GLenum =          0x0DF1;
pub const GL_FEEDBACK_BUFFER_TYPE : GLenum =          0x0DF2;
pub const GL_SELECTION_BUFFER_POINTER : GLenum =      0x0DF3;
pub const GL_SELECTION_BUFFER_SIZE : GLenum =         0x0DF4;
/*      GL_TEXTURE_BINDING_1D */
/*      GL_TEXTURE_BINDING_2D */
/*      GL_TEXTURE_BINDING_3D */
/*      GL_VERTEX_ARRAY */
/*      GL_NORMAL_ARRAY */
/*      GL_COLOR_ARRAY */
/*      GL_INDEX_ARRAY */
/*      GL_TEXTURE_COORD_ARRAY */
/*      GL_EDGE_FLAG_ARRAY */
/*      GL_VERTEX_ARRAY_SIZE */
/*      GL_VERTEX_ARRAY_TYPE */
/*      GL_VERTEX_ARRAY_STRIDE */
/*      GL_NORMAL_ARRAY_TYPE */
/*      GL_NORMAL_ARRAY_STRIDE */
/*      GL_COLOR_ARRAY_SIZE */
/*      GL_COLOR_ARRAY_TYPE */
/*      GL_COLOR_ARRAY_STRIDE */
/*      GL_INDEX_ARRAY_TYPE */
/*      GL_INDEX_ARRAY_STRIDE */
/*      GL_TEXTURE_COORD_ARRAY_SIZE */
/*      GL_TEXTURE_COORD_ARRAY_TYPE */
/*      GL_TEXTURE_COORD_ARRAY_STRIDE */
/*      GL_EDGE_FLAG_ARRAY_STRIDE */
/*      GL_POLYGON_OFFSET_FACTOR */
/*      GL_POLYGON_OFFSET_UNITS */
/*      GL_COLOR_TABLE */
/*      GL_POST_CONVOLUTION_COLOR_TABLE */
/*      GL_POST_COLOR_MATRIX_COLOR_TABLE */
/*      GL_CONVOLUTION_1D */
/*      GL_CONVOLUTION_2D */
/*      GL_SEPARABLE_2D */
/*      GL_POST_CONVOLUTION_RED_SCALE */
/*      GL_POST_CONVOLUTION_GREEN_SCALE */
/*      GL_POST_CONVOLUTION_BLUE_SCALE */
/*      GL_POST_CONVOLUTION_ALPHA_SCALE */
/*      GL_POST_CONVOLUTION_RED_BIAS */
/*      GL_POST_CONVOLUTION_GREEN_BIAS */
/*      GL_POST_CONVOLUTION_BLUE_BIAS */
/*      GL_POST_CONVOLUTION_ALPHA_BIAS */
/*      GL_COLOR_MATRIX */
/*      GL_COLOR_MATRIX_STACK_DEPTH */
/*      GL_MAX_COLOR_MATRIX_STACK_DEPTH */
/*      GL_POST_COLOR_MATRIX_RED_SCALE */
/*      GL_POST_COLOR_MATRIX_GREEN_SCALE */
/*      GL_POST_COLOR_MATRIX_BLUE_SCALE */
/*      GL_POST_COLOR_MATRIX_ALPHA_SCALE */
/*      GL_POST_COLOR_MATRIX_RED_BIAS */
/*      GL_POST_COLOR_MATRIX_GREEN_BIAS */
/*      GL_POST_COLOR_MATRIX_BLUE_BIAS */
/*      GL_POST_COLOR_MATRIX_ALPHA_BIAS */
/*      GL_HISTOGRAM */
/*      GL_MINMAX */
/*      GL_MAX_ELEMENTS_VERTICES */
/*      GL_MAX_ELEMENTS_INDICES */
/*      GL_RESCALE_NORMAL */
/*      GL_LIGHT_MODEL_COLOR_CONTROL */
/*      GL_PACK_SKIP_IMAGES */
/*      GL_PACK_IMAGE_HEIGHT */
/*      GL_UNPACK_SKIP_IMAGES */
/*      GL_UNPACK_IMAGE_HEIGHT */
/*      GL_TEXTURE_3D */
/*      GL_MAX_3D_TEXTURE_SIZE */
/*      GL_BLEND_COLOR */
/*      GL_BLEND_EQUATION */

/* GetTextureParameter */
/*      GL_TEXTURE_MAG_FILTER */
/*      GL_TEXTURE_MIN_FILTER */
/*      GL_TEXTURE_WRAP_S */
/*      GL_TEXTURE_WRAP_T */
pub const GL_TEXTURE_WIDTH : GLenum =                 0x1000;
pub const GL_TEXTURE_HEIGHT : GLenum =                0x1001;
pub const GL_TEXTURE_INTERNAL_FORMAT : GLenum =       0x1003;
pub const GL_TEXTURE_BORDER_COLOR : GLenum =          0x1004;
pub const GL_TEXTURE_BORDER : GLenum =                0x1005;
/*      GL_TEXTURE_RED_SIZE */
/*      GL_TEXTURE_GREEN_SIZE */
/*      GL_TEXTURE_BLUE_SIZE */
/*      GL_TEXTURE_ALPHA_SIZE */
/*      GL_TEXTURE_LUMINANCE_SIZE */
/*      GL_TEXTURE_INTENSITY_SIZE */
/*      GL_TEXTURE_PRIORITY */
/*      GL_TEXTURE_RESIDENT */
/*      GL_TEXTURE_DEPTH */
/*      GL_TEXTURE_WRAP_R */
/*      GL_TEXTURE_MIN_LOD */
/*      GL_TEXTURE_MAX_LOD */
/*      GL_TEXTURE_BASE_LEVEL */
/*      GL_TEXTURE_MAX_LEVEL */

/* HintMode */
pub const GL_DONT_CARE : GLenum =                     0x1100;
pub const GL_FASTEST : GLenum =                       0x1101;
pub const GL_NICEST : GLenum =                        0x1102;

/* HintTarget */
/*      GL_PERSPECTIVE_CORRECTION_HINT */
/*      GL_POINT_SMOOTH_HINT */
/*      GL_LINE_SMOOTH_HINT */
/*      GL_POLYGON_SMOOTH_HINT */
/*      GL_FOG_HINT */

/* HistogramTarget */
/*      GL_HISTOGRAM */
/*      GL_PROXY_HISTOGRAM */

/* IndexPointerType */
/*      GL_SHORT */
/*      GL_INT */
/*      GL_FLOAT */
/*      GL_DOUBLE */

/* LightModelColorControl */
/*      GL_SINGLE_COLOR */
/*      GL_SEPARATE_SPECULAR_COLOR */

/* LightModelParameter */
/*      GL_LIGHT_MODEL_AMBIENT */
/*      GL_LIGHT_MODEL_LOCAL_VIEWER */
/*      GL_LIGHT_MODEL_TWO_SIDE */
/*      GL_LIGHT_MODEL_COLOR_CONTROL */

/* LightName */
pub const GL_LIGHT0 : GLenum =                        0x4000;
pub const GL_LIGHT1 : GLenum =                        0x4001;
pub const GL_LIGHT2 : GLenum =                        0x4002;
pub const GL_LIGHT3 : GLenum =                        0x4003;
pub const GL_LIGHT4 : GLenum =                        0x4004;
pub const GL_LIGHT5 : GLenum =                        0x4005;
pub const GL_LIGHT6 : GLenum =                        0x4006;
pub const GL_LIGHT7 : GLenum =                        0x4007;

/* LightParameter */
pub const GL_AMBIENT : GLenum =                       0x1200;
pub const GL_DIFFUSE : GLenum =                       0x1201;
pub const GL_SPECULAR : GLenum =                      0x1202;
pub const GL_POSITION : GLenum =                      0x1203;
pub const GL_SPOT_DIRECTION : GLenum =                0x1204;
pub const GL_SPOT_EXPONENT : GLenum =                 0x1205;
pub const GL_SPOT_CUTOFF : GLenum =                   0x1206;
pub const GL_CONSTANT_ATTENUATION : GLenum =          0x1207;
pub const GL_LINEAR_ATTENUATION : GLenum =            0x1208;
pub const GL_QUADRATIC_ATTENUATION : GLenum =         0x1209;

/* InterleavedArrays */
/*      GL_V2F */
/*      GL_V3F */
/*      GL_C4UB_V2F */
/*      GL_C4UB_V3F */
/*      GL_C3F_V3F */
/*      GL_N3F_V3F */
/*      GL_C4F_N3F_V3F */
/*      GL_T2F_V3F */
/*      GL_T4F_V4F */
/*      GL_T2F_C4UB_V3F */
/*      GL_T2F_C3F_V3F */
/*      GL_T2F_N3F_V3F */
/*      GL_T2F_C4F_N3F_V3F */
/*      GL_T4F_C4F_N3F_V4F */

/* ListMode */
pub const GL_COMPILE : GLenum =                       0x1300;
pub const GL_COMPILE_AND_EXECUTE : GLenum =           0x1301;

/* ListNameType */
/*      GL_BYTE */
/*      GL_UNSIGNED_BYTE */
/*      GL_SHORT */
/*      GL_UNSIGNED_SHORT */
/*      GL_INT */
/*      GL_UNSIGNED_INT */
/*      GL_FLOAT */
/*      GL_2_BYTES */
/*      GL_3_BYTES */
/*      GL_4_BYTES */

/* LogicOp */
pub const GL_CLEAR : GLenum =                         0x1500;
pub const GL_AND : GLenum =                           0x1501;
pub const GL_AND_REVERSE : GLenum =                   0x1502;
pub const GL_COPY : GLenum =                          0x1503;
pub const GL_AND_INVERTED : GLenum =                  0x1504;
pub const GL_NOOP : GLenum =                          0x1505;
pub const GL_XOR : GLenum =                           0x1506;
pub const GL_OR : GLenum =                            0x1507;
pub const GL_NOR : GLenum =                           0x1508;
pub const GL_EQUIV : GLenum =                         0x1509;
pub const GL_INVERT : GLenum =                        0x150A;
pub const GL_OR_REVERSE : GLenum =                    0x150B;
pub const GL_COPY_INVERTED : GLenum =                 0x150C;
pub const GL_OR_INVERTED : GLenum =                   0x150D;
pub const GL_NAND : GLenum =                          0x150E;
pub const GL_SET : GLenum =                           0x150F;

/* MapTarget */
/*      GL_MAP1_COLOR_4 */
/*      GL_MAP1_INDEX */
/*      GL_MAP1_NORMAL */
/*      GL_MAP1_TEXTURE_COORD_1 */
/*      GL_MAP1_TEXTURE_COORD_2 */
/*      GL_MAP1_TEXTURE_COORD_3 */
/*      GL_MAP1_TEXTURE_COORD_4 */
/*      GL_MAP1_VERTEX_3 */
/*      GL_MAP1_VERTEX_4 */
/*      GL_MAP2_COLOR_4 */
/*      GL_MAP2_INDEX */
/*      GL_MAP2_NORMAL */
/*      GL_MAP2_TEXTURE_COORD_1 */
/*      GL_MAP2_TEXTURE_COORD_2 */
/*      GL_MAP2_TEXTURE_COORD_3 */
/*      GL_MAP2_TEXTURE_COORD_4 */
/*      GL_MAP2_VERTEX_3 */
/*      GL_MAP2_VERTEX_4 */

/* MaterialFace */
/*      GL_FRONT */
/*      GL_BACK */
/*      GL_FRONT_AND_BACK */

/* MaterialParameter */
pub const GL_EMISSION : GLenum =                      0x1600;
pub const GL_SHININESS : GLenum =                     0x1601;
pub const GL_AMBIENT_AND_DIFFUSE : GLenum =           0x1602;
pub const GL_COLOR_INDEXES : GLenum =                 0x1603;
/*      GL_AMBIENT */
/*      GL_DIFFUSE */
/*      GL_SPECULAR */

/* MatrixMode */
pub const GL_MODELVIEW : GLenum =                     0x1700;
pub const GL_PROJECTION : GLenum =                    0x1701;
pub const GL_TEXTURE : GLenum =                       0x1702;

/* MeshMode1 */
/*      GL_POINT */
/*      GL_LINE */

/* MeshMode2 */
/*      GL_POINT */
/*      GL_LINE */
/*      GL_FILL */

/* MinmaxTarget */
/*      GL_MINMAX */

/* NormalPointerType */
/*      GL_BYTE */
/*      GL_SHORT */
/*      GL_INT */
/*      GL_FLOAT */
/*      GL_DOUBLE */

/* PixelCopyType */
pub const GL_COLOR : GLenum =                         0x1800;
pub const GL_DEPTH : GLenum =                         0x1801;
pub const GL_STENCIL : GLenum =                       0x1802;

/* PixelFormat */
pub const GL_COLOR_INDEX : GLenum =                   0x1900;
pub const GL_STENCIL_INDEX : GLenum =                 0x1901;
pub const GL_DEPTH_COMPONENT : GLenum =               0x1902;
pub const GL_RED : GLenum =                           0x1903;
pub const GL_GREEN : GLenum =                         0x1904;
pub const GL_BLUE : GLenum =                          0x1905;
pub const GL_ALPHA : GLenum =                         0x1906;
pub const GL_RGB : GLenum =                           0x1907;
pub const GL_RGBA : GLenum =                          0x1908;
pub const GL_LUMINANCE : GLenum =                     0x1909;
pub const GL_LUMINANCE_ALPHA : GLenum =               0x190A;
/*      GL_ABGR */

/* PixelInternalFormat */
/*      GL_ALPHA4 */
/*      GL_ALPHA8 */
/*      GL_ALPHA12 */
/*      GL_ALPHA16 */
/*      GL_LUMINANCE4 */
/*      GL_LUMINANCE8 */
/*      GL_LUMINANCE12 */
/*      GL_LUMINANCE16 */
/*      GL_LUMINANCE4_ALPHA4 */
/*      GL_LUMINANCE6_ALPHA2 */
/*      GL_LUMINANCE8_ALPHA8 */
/*      GL_LUMINANCE12_ALPHA4 */
/*      GL_LUMINANCE12_ALPHA12 */
/*      GL_LUMINANCE16_ALPHA16 */
/*      GL_INTENSITY */
/*      GL_INTENSITY4 */
/*      GL_INTENSITY8 */
/*      GL_INTENSITY12 */
/*      GL_INTENSITY16 */
/*      GL_R3_G3_B2 */
/*      GL_RGB4 */
/*      GL_RGB5 */
/*      GL_RGB8 */
/*      GL_RGB10 */
/*      GL_RGB12 */
/*      GL_RGB16 */
/*      GL_RGBA2 */
/*      GL_RGBA4 */
/*      GL_RGB5_A1 */
/*      GL_RGBA8 */
/*      GL_RGB10_A2 */
/*      GL_RGBA12 */
/*      GL_RGBA16 */

/* PixelMap */
/*      GL_PIXEL_MAP_I_TO_I */
/*      GL_PIXEL_MAP_S_TO_S */
/*      GL_PIXEL_MAP_I_TO_R */
/*      GL_PIXEL_MAP_I_TO_G */
/*      GL_PIXEL_MAP_I_TO_B */
/*      GL_PIXEL_MAP_I_TO_A */
/*      GL_PIXEL_MAP_R_TO_R */
/*      GL_PIXEL_MAP_G_TO_G */
/*      GL_PIXEL_MAP_B_TO_B */
/*      GL_PIXEL_MAP_A_TO_A */

/* PixelStore */
/*      GL_UNPACK_SWAP_BYTES */
/*      GL_UNPACK_LSB_FIRST */
/*      GL_UNPACK_ROW_LENGTH */
/*      GL_UNPACK_SKIP_ROWS */
/*      GL_UNPACK_SKIP_PIXELS */
/*      GL_UNPACK_ALIGNMENT */
/*      GL_PACK_SWAP_BYTES */
/*      GL_PACK_LSB_FIRST */
/*      GL_PACK_ROW_LENGTH */
/*      GL_PACK_SKIP_ROWS */
/*      GL_PACK_SKIP_PIXELS */
/*      GL_PACK_ALIGNMENT */
/*      GL_PACK_SKIP_IMAGES */
/*      GL_PACK_IMAGE_HEIGHT */
/*      GL_UNPACK_SKIP_IMAGES */
/*      GL_UNPACK_IMAGE_HEIGHT */

/* PixelTransfer */
/*      GL_MAP_COLOR */
/*      GL_MAP_STENCIL */
/*      GL_INDEX_SHIFT */
/*      GL_INDEX_OFFSET */
/*      GL_RED_SCALE */
/*      GL_RED_BIAS */
/*      GL_GREEN_SCALE */
/*      GL_GREEN_BIAS */
/*      GL_BLUE_SCALE */
/*      GL_BLUE_BIAS */
/*      GL_ALPHA_SCALE */
/*      GL_ALPHA_BIAS */
/*      GL_DEPTH_SCALE */
/*      GL_DEPTH_BIAS */
/*      GL_POST_CONVOLUTION_RED_SCALE */
/*      GL_POST_CONVOLUTION_GREEN_SCALE */
/*      GL_POST_CONVOLUTION_BLUE_SCALE */
/*      GL_POST_CONVOLUTION_ALPHA_SCALE */
/*      GL_POST_CONVOLUTION_RED_BIAS */
/*      GL_POST_CONVOLUTION_GREEN_BIAS */
/*      GL_POST_CONVOLUTION_BLUE_BIAS */
/*      GL_POST_CONVOLUTION_ALPHA_BIAS */
/*      GL_POST_COLOR_MATRIX_RED_SCALE */
/*      GL_POST_COLOR_MATRIX_GREEN_SCALE */
/*      GL_POST_COLOR_MATRIX_BLUE_SCALE */
/*      GL_POST_COLOR_MATRIX_ALPHA_SCALE */
/*      GL_POST_COLOR_MATRIX_RED_BIAS */
/*      GL_POST_COLOR_MATRIX_GREEN_BIAS */
/*      GL_POST_COLOR_MATRIX_BLUE_BIAS */
/*      GL_POST_COLOR_MATRIX_ALPHA_BIAS */

/* PixelType */
pub const GL_BITMAP : GLenum =                        0x1A00;
/*      GL_BYTE */
/*      GL_UNSIGNED_BYTE */
/*      GL_SHORT */
/*      GL_UNSIGNED_SHORT */
/*      GL_INT */
/*      GL_UNSIGNED_INT */
/*      GL_FLOAT */
/*      GL_BGR */
/*      GL_BGRA */
/*      GL_UNSIGNED_BYTE_3_3_2 */
/*      GL_UNSIGNED_SHORT_4_4_4_4 */
/*      GL_UNSIGNED_SHORT_5_5_5_1 */
/*      GL_UNSIGNED_INT_8_8_8_8 */
/*      GL_UNSIGNED_INT_10_10_10_2 */
/*      GL_UNSIGNED_SHORT_5_6_5 */
/*      GL_UNSIGNED_BYTE_2_3_3_REV */
/*      GL_UNSIGNED_SHORT_5_6_5_REV */
/*      GL_UNSIGNED_SHORT_4_4_4_4_REV */
/*      GL_UNSIGNED_SHORT_1_5_5_5_REV */
/*      GL_UNSIGNED_INT_8_8_8_8_REV */
/*      GL_UNSIGNED_INT_2_10_10_10_REV */

/* PolygonMode */
pub const GL_POINT : GLenum =                         0x1B00;
pub const GL_LINE : GLenum =                          0x1B01;
pub const GL_FILL : GLenum =                          0x1B02;

/* ReadBufferMode */
/*      GL_FRONT_LEFT */
/*      GL_FRONT_RIGHT */
/*      GL_BACK_LEFT */
/*      GL_BACK_RIGHT */
/*      GL_FRONT */
/*      GL_BACK */
/*      GL_LEFT */
/*      GL_RIGHT */
/*      GL_AUX0 */
/*      GL_AUX1 */
/*      GL_AUX2 */
/*      GL_AUX3 */

/* RenderingMode */
pub const GL_RENDER : GLenum =                        0x1C00;
pub const GL_FEEDBACK : GLenum =                      0x1C01;
pub const GL_SELECT : GLenum =                        0x1C02;

/* SeparableTarget */
/*      GL_SEPARABLE_2D */

/* ShadingModel */
pub const GL_FLAT : GLenum =                          0x1D00;
pub const GL_SMOOTH : GLenum =                        0x1D01;

/* StencilFunction */
/*      GL_NEVER */
/*      GL_LESS */
/*      GL_EQUAL */
/*      GL_LEQUAL */
/*      GL_GREATER */
/*      GL_NOTEQUAL */
/*      GL_GEQUAL */
/*      GL_ALWAYS */

/* StencilOp */
/*      GL_ZERO */
pub const GL_KEEP : GLenum =                          0x1E00;
pub const GL_REPLACE : GLenum =                       0x1E01;
pub const GL_INCR : GLenum =                          0x1E02;
pub const GL_DECR : GLenum =                          0x1E03;
/*      GL_INVERT */

/* StringName */
pub const GL_VENDOR : GLenum =                        0x1F00;
pub const GL_RENDERER : GLenum =                      0x1F01;
pub const GL_VERSION : GLenum =                       0x1F02;
pub const GL_EXTENSIONS : GLenum =                    0x1F03;

/* TextureCoordName */
pub const GL_S : GLenum =                             0x2000;
pub const GL_T : GLenum =                             0x2001;
pub const GL_R : GLenum =                             0x2002;
pub const GL_Q : GLenum =                             0x2003;

/* TexCoordPointerType */
/*      GL_SHORT */
/*      GL_INT */
/*      GL_FLOAT */
/*      GL_DOUBLE */

/* TextureEnvMode */
pub const GL_MODULATE : GLenum =                      0x2100;
pub const GL_DECAL : GLenum =                         0x2101;
/*      GL_BLEND */
/*      GL_REPLACE */

/* TextureEnvParameter */
pub const GL_TEXTURE_ENV_MODE : GLenum =              0x2200;
pub const GL_TEXTURE_ENV_COLOR : GLenum =             0x2201;

/* TextureEnvTarget */
pub const GL_TEXTURE_ENV : GLenum =                   0x2300;

/* TextureGenMode */
pub const GL_EYE_LINEAR : GLenum =                    0x2400;
pub const GL_OBJECT_LINEAR : GLenum =                 0x2401;
pub const GL_SPHERE_MAP : GLenum =                    0x2402;

/* TextureGenParameter */
pub const GL_TEXTURE_GEN_MODE : GLenum =              0x2500;
pub const GL_OBJECT_PLANE : GLenum =                  0x2501;
pub const GL_EYE_PLANE : GLenum =                     0x2502;

/* TextureMagFilter */
pub const GL_NEAREST : GLenum =                       0x2600;
pub const GL_LINEAR : GLenum =                        0x2601;

/* TextureMinFilter */
/*      GL_NEAREST */
/*      GL_LINEAR */
pub const GL_NEAREST_MIPMAP_NEAREST : GLenum =        0x2700;
pub const GL_LINEAR_MIPMAP_NEAREST : GLenum =         0x2701;
pub const GL_NEAREST_MIPMAP_LINEAR : GLenum =         0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR : GLenum =          0x2703;

/* TextureParameterName */
pub const GL_TEXTURE_MAG_FILTER : GLenum =            0x2800;
pub const GL_TEXTURE_MIN_FILTER : GLenum =            0x2801;
pub const GL_TEXTURE_WRAP_S : GLenum =                0x2802;
pub const GL_TEXTURE_WRAP_T : GLenum =                0x2803;
/*      GL_TEXTURE_BORDER_COLOR */
/*      GL_TEXTURE_PRIORITY */
/*      GL_TEXTURE_WRAP_R */
/*      GL_TEXTURE_MIN_LOD */
/*      GL_TEXTURE_MAX_LOD */
/*      GL_TEXTURE_BASE_LEVEL */
/*      GL_TEXTURE_MAX_LEVEL */

/* TextureTarget */
/*      GL_TEXTURE_1D */
/*      GL_TEXTURE_2D */
/*      GL_PROXY_TEXTURE_1D */
/*      GL_PROXY_TEXTURE_2D */
/*      GL_TEXTURE_3D */
/*      GL_PROXY_TEXTURE_3D */

/* TextureWrapMode */
pub const GL_CLAMP : GLenum =                         0x2900;
pub const GL_REPEAT : GLenum =                        0x2901;
/*      GL_CLAMP_TO_EDGE */

/* VertexPointerType */
/*      GL_SHORT */
/*      GL_INT */
/*      GL_FLOAT */
/*      GL_DOUBLE */

/* ClientAttribMask */
pub const GL_CLIENT_PIXEL_STORE_BIT : GLenum =        0x00000001;
pub const GL_CLIENT_VERTEX_ARRAY_BIT : GLenum =       0x00000002;
pub const GL_CLIENT_ALL_ATTRIB_BITS : GLenum =        0xffffffff;

/* polygon_offset */
pub const GL_POLYGON_OFFSET_FACTOR : GLenum =         0x8038;
pub const GL_POLYGON_OFFSET_UNITS : GLenum =          0x2A00;
pub const GL_POLYGON_OFFSET_POINT : GLenum =          0x2A01;
pub const GL_POLYGON_OFFSET_LINE : GLenum =           0x2A02;
pub const GL_POLYGON_OFFSET_FILL : GLenum =           0x8037;

/* texture */
pub const GL_ALPHA4 : GLenum =                        0x803B;
pub const GL_ALPHA8 : GLenum =                        0x803C;
pub const GL_ALPHA12 : GLenum =                       0x803D;
pub const GL_ALPHA16 : GLenum =                       0x803E;
pub const GL_LUMINANCE4 : GLenum =                    0x803F;
pub const GL_LUMINANCE8 : GLenum =                    0x8040;
pub const GL_LUMINANCE12 : GLenum =                   0x8041;
pub const GL_LUMINANCE16 : GLenum =                   0x8042;
pub const GL_LUMINANCE4_ALPHA4 : GLenum =             0x8043;
pub const GL_LUMINANCE6_ALPHA2 : GLenum =             0x8044;
pub const GL_LUMINANCE8_ALPHA8 : GLenum =             0x8045;
pub const GL_LUMINANCE12_ALPHA4 : GLenum =            0x8046;
pub const GL_LUMINANCE12_ALPHA12 : GLenum =           0x8047;
pub const GL_LUMINANCE16_ALPHA16 : GLenum =           0x8048;
pub const GL_INTENSITY : GLenum =                     0x8049;
pub const GL_INTENSITY4 : GLenum =                    0x804A;
pub const GL_INTENSITY8 : GLenum =                    0x804B;
pub const GL_INTENSITY12 : GLenum =                   0x804C;
pub const GL_INTENSITY16 : GLenum =                   0x804D;
pub const GL_R3_G3_B2 : GLenum =                      0x2A10;
pub const GL_RGB4 : GLenum =                          0x804F;
pub const GL_RGB5 : GLenum =                          0x8050;
pub const GL_RGB8 : GLenum =                          0x8051;
pub const GL_RGB10 : GLenum =                         0x8052;
pub const GL_RGB12 : GLenum =                         0x8053;
pub const GL_RGB16 : GLenum =                         0x8054;
pub const GL_RGBA2 : GLenum =                         0x8055;
pub const GL_RGBA4 : GLenum =                         0x8056;
pub const GL_RGB5_A1 : GLenum =                       0x8057;
pub const GL_RGBA8 : GLenum =                         0x8058;
pub const GL_RGB10_A2 : GLenum =                      0x8059;
pub const GL_RGBA12 : GLenum =                        0x805A;
pub const GL_RGBA16 : GLenum =                        0x805B;
pub const GL_TEXTURE_RED_SIZE : GLenum =              0x805C;
pub const GL_TEXTURE_GREEN_SIZE : GLenum =            0x805D;
pub const GL_TEXTURE_BLUE_SIZE : GLenum =             0x805E;
pub const GL_TEXTURE_ALPHA_SIZE : GLenum =            0x805F;
pub const GL_TEXTURE_LUMINANCE_SIZE : GLenum =        0x8060;
pub const GL_TEXTURE_INTENSITY_SIZE : GLenum =        0x8061;
pub const GL_PROXY_TEXTURE_1D : GLenum =              0x8063;
pub const GL_PROXY_TEXTURE_2D : GLenum =              0x8064;

/* texture_object */
pub const GL_TEXTURE_PRIORITY : GLenum =              0x8066;
pub const GL_TEXTURE_RESIDENT : GLenum =              0x8067;
pub const GL_TEXTURE_BINDING_1D : GLenum =            0x8068;
pub const GL_TEXTURE_BINDING_2D : GLenum =            0x8069;
pub const GL_TEXTURE_BINDING_3D : GLenum =            0x806A;

/* vertex_array */
pub const GL_VERTEX_ARRAY : GLenum =                  0x8074;
pub const GL_NORMAL_ARRAY : GLenum =                  0x8075;
pub const GL_COLOR_ARRAY : GLenum =                   0x8076;
pub const GL_INDEX_ARRAY : GLenum =                   0x8077;
pub const GL_TEXTURE_COORD_ARRAY : GLenum =           0x8078;
pub const GL_EDGE_FLAG_ARRAY : GLenum =               0x8079;
pub const GL_VERTEX_ARRAY_SIZE : GLenum =             0x807A;
pub const GL_VERTEX_ARRAY_TYPE : GLenum =             0x807B;
pub const GL_VERTEX_ARRAY_STRIDE : GLenum =           0x807C;
pub const GL_NORMAL_ARRAY_TYPE : GLenum =             0x807E;
pub const GL_NORMAL_ARRAY_STRIDE : GLenum =           0x807F;
pub const GL_COLOR_ARRAY_SIZE : GLenum =              0x8081;
pub const GL_COLOR_ARRAY_TYPE : GLenum =              0x8082;
pub const GL_COLOR_ARRAY_STRIDE : GLenum =            0x8083;
pub const GL_INDEX_ARRAY_TYPE : GLenum =              0x8085;
pub const GL_INDEX_ARRAY_STRIDE : GLenum =            0x8086;
pub const GL_TEXTURE_COORD_ARRAY_SIZE : GLenum =      0x8088;
pub const GL_TEXTURE_COORD_ARRAY_TYPE : GLenum =      0x8089;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE : GLenum =    0x808A;
pub const GL_EDGE_FLAG_ARRAY_STRIDE : GLenum =        0x808C;
pub const GL_VERTEX_ARRAY_POINTER : GLenum =          0x808E;
pub const GL_NORMAL_ARRAY_POINTER : GLenum =          0x808F;
pub const GL_COLOR_ARRAY_POINTER : GLenum =           0x8090;
pub const GL_INDEX_ARRAY_POINTER : GLenum =           0x8091;
pub const GL_TEXTURE_COORD_ARRAY_POINTER : GLenum =   0x8092;
pub const GL_EDGE_FLAG_ARRAY_POINTER : GLenum =       0x8093;
pub const GL_V2F : GLenum =                           0x2A20;
pub const GL_V3F : GLenum =                           0x2A21;
pub const GL_C4UB_V2F : GLenum =                      0x2A22;
pub const GL_C4UB_V3F : GLenum =                      0x2A23;
pub const GL_C3F_V3F : GLenum =                       0x2A24;
pub const GL_N3F_V3F : GLenum =                       0x2A25;
pub const GL_C4F_N3F_V3F : GLenum =                   0x2A26;
pub const GL_T2F_V3F : GLenum =                       0x2A27;
pub const GL_T4F_V4F : GLenum =                       0x2A28;
pub const GL_T2F_C4UB_V3F : GLenum =                  0x2A29;
pub const GL_T2F_C3F_V3F : GLenum =                   0x2A2A;
pub const GL_T2F_N3F_V3F : GLenum =                   0x2A2B;
pub const GL_T2F_C4F_N3F_V3F : GLenum =               0x2A2C;
pub const GL_T4F_C4F_N3F_V4F : GLenum =               0x2A2D;

/* bgra */
pub const GL_BGR : GLenum =                           0x80E0;
pub const GL_BGRA : GLenum =                          0x80E1;

/* blend_color */
pub const GL_CONSTANT_COLOR : GLenum =                0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR : GLenum =      0x8002;
pub const GL_CONSTANT_ALPHA : GLenum =                0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA : GLenum =      0x8004;
pub const GL_BLEND_COLOR : GLenum =                   0x8005;

/* blend_minmax */
pub const GL_FUNC_ADD : GLenum =                      0x8006;
pub const GL_MIN : GLenum =                           0x8007;
pub const GL_MAX : GLenum =                           0x8008;
pub const GL_BLEND_EQUATION : GLenum =                0x8009;

/* blend_equation_separate */
pub const GL_BLEND_EQUATION_RGB : GLenum =            0x8009;
pub const GL_BLEND_EQUATION_ALPHA : GLenum =          0x883D;

/* blend_subtract */
pub const GL_FUNC_SUBTRACT : GLenum =                 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT : GLenum =         0x800B;

/* color_matrix */
pub const GL_COLOR_MATRIX : GLenum =                  0x80B1;
pub const GL_COLOR_MATRIX_STACK_DEPTH : GLenum =      0x80B2;
pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH : GLenum =  0x80B3;
pub const GL_POST_COLOR_MATRIX_RED_SCALE : GLenum =   0x80B4;
pub const GL_POST_COLOR_MATRIX_GREEN_SCALE : GLenum = 0x80B5;
pub const GL_POST_COLOR_MATRIX_BLUE_SCALE : GLenum =  0x80B6;
pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE : GLenum = 0x80B7;
pub const GL_POST_COLOR_MATRIX_RED_BIAS : GLenum =    0x80B8;
pub const GL_POST_COLOR_MATRIX_GREEN_BIAS : GLenum =  0x80B9;
pub const GL_POST_COLOR_MATRIX_BLUE_BIAS : GLenum =   0x80BA;
pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS : GLenum =  0x80BB;

/* color_table */
pub const GL_COLOR_TABLE : GLenum =                   0x80D0;
pub const GL_POST_CONVOLUTION_COLOR_TABLE : GLenum =  0x80D1;
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE : GLenum = 0x80D2;
pub const GL_PROXY_COLOR_TABLE : GLenum =             0x80D3;
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE : GLenum=0x80D4;
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE : GLenum=0x80D5;
pub const GL_COLOR_TABLE_SCALE : GLenum =             0x80D6;
pub const GL_COLOR_TABLE_BIAS : GLenum =              0x80D7;
pub const GL_COLOR_TABLE_FORMAT : GLenum =            0x80D8;
pub const GL_COLOR_TABLE_WIDTH : GLenum =             0x80D9;
pub const GL_COLOR_TABLE_RED_SIZE : GLenum =          0x80DA;
pub const GL_COLOR_TABLE_GREEN_SIZE : GLenum =        0x80DB;
pub const GL_COLOR_TABLE_BLUE_SIZE : GLenum =         0x80DC;
pub const GL_COLOR_TABLE_ALPHA_SIZE : GLenum =        0x80DD;
pub const GL_COLOR_TABLE_LUMINANCE_SIZE : GLenum =    0x80DE;
pub const GL_COLOR_TABLE_INTENSITY_SIZE : GLenum =    0x80DF;

/* convolution */
pub const GL_CONVOLUTION_1D : GLenum =                0x8010;
pub const GL_CONVOLUTION_2D : GLenum =                0x8011;
pub const GL_SEPARABLE_2D : GLenum =                  0x8012;
pub const GL_CONVOLUTION_BORDER_MODE : GLenum =       0x8013;
pub const GL_CONVOLUTION_FILTER_SCALE : GLenum =      0x8014;
pub const GL_CONVOLUTION_FILTER_BIAS : GLenum =       0x8015;
pub const GL_REDUCE : GLenum =                        0x8016;
pub const GL_CONVOLUTION_FORMAT : GLenum =            0x8017;
pub const GL_CONVOLUTION_WIDTH : GLenum =             0x8018;
pub const GL_CONVOLUTION_HEIGHT : GLenum =            0x8019;
pub const GL_MAX_CONVOLUTION_WIDTH : GLenum =         0x801A;
pub const GL_MAX_CONVOLUTION_HEIGHT : GLenum =        0x801B;
pub const GL_POST_CONVOLUTION_RED_SCALE : GLenum =    0x801C;
pub const GL_POST_CONVOLUTION_GREEN_SCALE : GLenum =  0x801D;
pub const GL_POST_CONVOLUTION_BLUE_SCALE : GLenum =   0x801E;
pub const GL_POST_CONVOLUTION_ALPHA_SCALE : GLenum =  0x801F;
pub const GL_POST_CONVOLUTION_RED_BIAS : GLenum =     0x8020;
pub const GL_POST_CONVOLUTION_GREEN_BIAS : GLenum =   0x8021;
pub const GL_POST_CONVOLUTION_BLUE_BIAS : GLenum =    0x8022;
pub const GL_POST_CONVOLUTION_ALPHA_BIAS : GLenum =   0x8023;
pub const GL_CONSTANT_BORDER : GLenum =               0x8151;
pub const GL_REPLICATE_BORDER : GLenum =              0x8153;
pub const GL_CONVOLUTION_BORDER_COLOR : GLenum =      0x8154;

/* draw_range_elements */
pub const GL_MAX_ELEMENTS_VERTICES : GLenum =         0x80E8;
pub const GL_MAX_ELEMENTS_INDICES : GLenum =          0x80E9;

/* histogram */
pub const GL_HISTOGRAM : GLenum =                     0x8024;
pub const GL_PROXY_HISTOGRAM : GLenum =               0x8025;
pub const GL_HISTOGRAM_WIDTH : GLenum =               0x8026;
pub const GL_HISTOGRAM_FORMAT : GLenum =              0x8027;
pub const GL_HISTOGRAM_RED_SIZE : GLenum =            0x8028;
pub const GL_HISTOGRAM_GREEN_SIZE : GLenum =          0x8029;
pub const GL_HISTOGRAM_BLUE_SIZE : GLenum =           0x802A;
pub const GL_HISTOGRAM_ALPHA_SIZE : GLenum =          0x802B;
pub const GL_HISTOGRAM_LUMINANCE_SIZE : GLenum =      0x802C;
pub const GL_HISTOGRAM_SINK : GLenum =                0x802D;
pub const GL_MINMAX : GLenum =                        0x802E;
pub const GL_MINMAX_FORMAT : GLenum =                 0x802F;
pub const GL_MINMAX_SINK : GLenum =                   0x8030;
pub const GL_TABLE_TOO_LARGE : GLenum =               0x8031;

/* packed_pixels */
pub const GL_UNSIGNED_BYTE_3_3_2 : GLenum =           0x8032;
pub const GL_UNSIGNED_SHORT_4_4_4_4 : GLenum =        0x8033;
pub const GL_UNSIGNED_SHORT_5_5_5_1 : GLenum =        0x8034;
pub const GL_UNSIGNED_INT_8_8_8_8 : GLenum =          0x8035;
pub const GL_UNSIGNED_INT_10_10_10_2 : GLenum =       0x8036;
pub const GL_UNSIGNED_BYTE_2_3_3_REV : GLenum =       0x8362;
pub const GL_UNSIGNED_SHORT_5_6_5 : GLenum =          0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV : GLenum =      0x8364;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV : GLenum =    0x8365;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV : GLenum =    0x8366;
pub const GL_UNSIGNED_INT_8_8_8_8_REV : GLenum =      0x8367;
pub const GL_UNSIGNED_INT_2_10_10_10_REV : GLenum =   0x8368;

/* rescale_normal */
pub const GL_RESCALE_NORMAL : GLenum =                0x803A;

/* separate_specular_color */
pub const GL_LIGHT_MODEL_COLOR_CONTROL : GLenum =     0x81F8;
pub const GL_SINGLE_COLOR : GLenum =                  0x81F9;
pub const GL_SEPARATE_SPECULAR_COLOR : GLenum =       0x81FA;

/* texture3D */
pub const GL_PACK_SKIP_IMAGES : GLenum =              0x806B;
pub const GL_PACK_IMAGE_HEIGHT : GLenum =             0x806C;
pub const GL_UNPACK_SKIP_IMAGES : GLenum =            0x806D;
pub const GL_UNPACK_IMAGE_HEIGHT : GLenum =           0x806E;
pub const GL_TEXTURE_3D : GLenum =                    0x806F;
pub const GL_PROXY_TEXTURE_3D : GLenum =              0x8070;
pub const GL_TEXTURE_DEPTH : GLenum =                 0x8071;
pub const GL_TEXTURE_WRAP_R : GLenum =                0x8072;
pub const GL_MAX_3D_TEXTURE_SIZE : GLenum =           0x8073;

/* texture_edge_clamp */
pub const GL_CLAMP_TO_EDGE : GLenum =                 0x812F;
pub const GL_CLAMP_TO_BORDER : GLenum =               0x812D;

/* texture_lod */
pub const GL_TEXTURE_MIN_LOD : GLenum =               0x813A;
pub const GL_TEXTURE_MAX_LOD : GLenum =               0x813B;
pub const GL_TEXTURE_BASE_LEVEL : GLenum =            0x813C;
pub const GL_TEXTURE_MAX_LEVEL : GLenum =             0x813D;

/* GetTarget1_2 */
pub const GL_SMOOTH_POINT_SIZE_RANGE : GLenum =       0x0B12;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY : GLenum = 0x0B13;
pub const GL_SMOOTH_LINE_WIDTH_RANGE : GLenum =       0x0B22;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY : GLenum = 0x0B23;
pub const GL_ALIASED_POINT_SIZE_RANGE : GLenum =      0x846D;
pub const GL_ALIASED_LINE_WIDTH_RANGE : GLenum =      0x846E;

pub const GL_TEXTURE0 : GLenum =                      0x84C0;
pub const GL_TEXTURE1 : GLenum =                      0x84C1;
pub const GL_TEXTURE2 : GLenum =                      0x84C2;
pub const GL_TEXTURE3 : GLenum =                      0x84C3;
pub const GL_TEXTURE4 : GLenum =                      0x84C4;
pub const GL_TEXTURE5 : GLenum =                      0x84C5;
pub const GL_TEXTURE6 : GLenum =                      0x84C6;
pub const GL_TEXTURE7 : GLenum =                      0x84C7;
pub const GL_TEXTURE8 : GLenum =                      0x84C8;
pub const GL_TEXTURE9 : GLenum =                      0x84C9;
pub const GL_TEXTURE10 : GLenum =                     0x84CA;
pub const GL_TEXTURE11 : GLenum =                     0x84CB;
pub const GL_TEXTURE12 : GLenum =                     0x84CC;
pub const GL_TEXTURE13 : GLenum =                     0x84CD;
pub const GL_TEXTURE14 : GLenum =                     0x84CE;
pub const GL_TEXTURE15 : GLenum =                     0x84CF;
pub const GL_TEXTURE16 : GLenum =                     0x84D0;
pub const GL_TEXTURE17 : GLenum =                     0x84D1;
pub const GL_TEXTURE18 : GLenum =                     0x84D2;
pub const GL_TEXTURE19 : GLenum =                     0x84D3;
pub const GL_TEXTURE20 : GLenum =                     0x84D4;
pub const GL_TEXTURE21 : GLenum =                     0x84D5;
pub const GL_TEXTURE22 : GLenum =                     0x84D6;
pub const GL_TEXTURE23 : GLenum =                     0x84D7;
pub const GL_TEXTURE24 : GLenum =                     0x84D8;
pub const GL_TEXTURE25 : GLenum =                     0x84D9;
pub const GL_TEXTURE26 : GLenum =                     0x84DA;
pub const GL_TEXTURE27 : GLenum =                     0x84DB;
pub const GL_TEXTURE28 : GLenum =                     0x84DC;
pub const GL_TEXTURE29 : GLenum =                     0x84DD;
pub const GL_TEXTURE30 : GLenum =                     0x84DE;
pub const GL_TEXTURE31 : GLenum =                     0x84DF;
pub const GL_ACTIVE_TEXTURE : GLenum =                0x84E0;
pub const GL_CLIENT_ACTIVE_TEXTURE : GLenum =         0x84E1;
pub const GL_MAX_TEXTURE_UNITS : GLenum =             0x84E2;

pub const GL_COMBINE : GLenum =                       0x8570;
pub const GL_COMBINE_RGB : GLenum =                   0x8571;
pub const GL_COMBINE_ALPHA : GLenum =                 0x8572;
pub const GL_RGB_SCALE : GLenum =                     0x8573;
pub const GL_ADD_SIGNED : GLenum =                    0x8574;
pub const GL_INTERPOLATE : GLenum =                   0x8575;
pub const GL_CONSTANT : GLenum =                      0x8576;
pub const GL_PRIMARY_COLOR : GLenum =                 0x8577;
pub const GL_PREVIOUS : GLenum =                      0x8578;
pub const GL_SUBTRACT : GLenum =                      0x84E7;

pub const GL_SRC0_RGB : GLenum =                      0x8580;
pub const GL_SRC1_RGB : GLenum =                      0x8581;
pub const GL_SRC2_RGB : GLenum =                      0x8582;
pub const GL_SRC0_ALPHA : GLenum =                    0x8588;
pub const GL_SRC1_ALPHA : GLenum =                    0x8589;
pub const GL_SRC2_ALPHA : GLenum =                    0x858A;

/* Obsolete */
pub const GL_SOURCE0_RGB : GLenum =                   0x8580;
pub const GL_SOURCE1_RGB : GLenum =                   0x8581;
pub const GL_SOURCE2_RGB : GLenum =                   0x8582;
pub const GL_SOURCE0_ALPHA : GLenum =                 0x8588;
pub const GL_SOURCE1_ALPHA : GLenum =                 0x8589;
pub const GL_SOURCE2_ALPHA : GLenum =                 0x858A;

pub const GL_OPERAND0_RGB : GLenum =                  0x8590;
pub const GL_OPERAND1_RGB : GLenum =                  0x8591;
pub const GL_OPERAND2_RGB : GLenum =                  0x8592;
pub const GL_OPERAND0_ALPHA : GLenum =                0x8598;
pub const GL_OPERAND1_ALPHA : GLenum =                0x8599;
pub const GL_OPERAND2_ALPHA : GLenum =                0x859A;

pub const GL_DOT3_RGB : GLenum =                      0x86AE;
pub const GL_DOT3_RGBA : GLenum =                     0x86AF;

pub const GL_TRANSPOSE_MODELVIEW_MATRIX : GLenum =    0x84E3;
pub const GL_TRANSPOSE_PROJECTION_MATRIX : GLenum =   0x84E4;
pub const GL_TRANSPOSE_TEXTURE_MATRIX : GLenum =      0x84E5;
pub const GL_TRANSPOSE_COLOR_MATRIX : GLenum =        0x84E6;

pub const GL_NORMAL_MAP : GLenum =                    0x8511;
pub const GL_REFLECTION_MAP : GLenum =                0x8512;
pub const GL_TEXTURE_CUBE_MAP : GLenum =              0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP : GLenum =      0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X : GLenum =   0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X : GLenum =   0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y : GLenum =   0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y : GLenum =   0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z : GLenum =   0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z : GLenum =   0x851A;
pub const GL_PROXY_TEXTURE_CUBE_MAP : GLenum =        0x851B;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE : GLenum =     0x851C;

pub const GL_COMPRESSED_ALPHA : GLenum =              0x84E9;
pub const GL_COMPRESSED_LUMINANCE : GLenum =          0x84EA;
pub const GL_COMPRESSED_LUMINANCE_ALPHA : GLenum =    0x84EB;
pub const GL_COMPRESSED_INTENSITY : GLenum =          0x84EC;
pub const GL_COMPRESSED_RGB : GLenum =                0x84ED;
pub const GL_COMPRESSED_RGBA : GLenum =               0x84EE;
pub const GL_TEXTURE_COMPRESSION_HINT : GLenum =      0x84EF;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE : GLenum = 0x86A0;
pub const GL_TEXTURE_COMPRESSED : GLenum =            0x86A1;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS : GLenum =0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS : GLenum =    0x86A3;

pub const GL_MULTISAMPLE : GLenum =                   0x809D;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE : GLenum =      0x809E;
pub const GL_SAMPLE_ALPHA_TO_ONE : GLenum =           0x809F;
pub const GL_SAMPLE_COVERAGE : GLenum =               0x80A0;
pub const GL_SAMPLE_BUFFERS : GLenum =                0x80A8;
pub const GL_SAMPLES : GLenum =                       0x80A9;
pub const GL_SAMPLE_COVERAGE_VALUE : GLenum =         0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT : GLenum =        0x80AB;
pub const GL_MULTISAMPLE_BIT : GLenum =               0x20000000;

pub const GL_DEPTH_COMPONENT16 : GLenum =             0x81A5;
pub const GL_DEPTH_COMPONENT24 : GLenum =             0x81A6;
pub const GL_DEPTH_COMPONENT32 : GLenum =             0x81A7;
pub const GL_TEXTURE_DEPTH_SIZE : GLenum =            0x884A;
pub const GL_DEPTH_TEXTURE_MODE : GLenum =            0x884B;

pub const GL_TEXTURE_COMPARE_MODE : GLenum =          0x884C;
pub const GL_TEXTURE_COMPARE_FUNC : GLenum =          0x884D;
pub const GL_COMPARE_R_TO_TEXTURE : GLenum =          0x884E;

/* occlusion_query */
pub const GL_QUERY_COUNTER_BITS : GLenum =            0x8864;
pub const GL_CURRENT_QUERY : GLenum =                 0x8865;
pub const GL_QUERY_RESULT : GLenum =                  0x8866;
pub const GL_QUERY_RESULT_AVAILABLE : GLenum =        0x8867;
pub const GL_SAMPLES_PASSED : GLenum =                0x8914;

pub const GL_FOG_COORD_SRC : GLenum =                 0x8450;
pub const GL_FOG_COORD : GLenum =                     0x8451;
pub const GL_FRAGMENT_DEPTH : GLenum =                0x8452;
pub const GL_CURRENT_FOG_COORD : GLenum =             0x8453  ;
pub const GL_FOG_COORD_ARRAY_TYPE : GLenum =          0x8454;
pub const GL_FOG_COORD_ARRAY_STRIDE : GLenum =        0x8455;
pub const GL_FOG_COORD_ARRAY_POINTER : GLenum =       0x8456;
pub const GL_FOG_COORD_ARRAY : GLenum =               0x8457;

/* Obsolete */
pub const GL_FOG_COORDINATE_SOURCE : GLenum =         0x8450;
pub const GL_FOG_COORDINATE : GLenum =                0x8451;
pub const GL_CURRENT_FOG_COORDINATE : GLenum =        0x8453  ;
pub const GL_FOG_COORDINATE_ARRAY_TYPE : GLenum =     0x8454;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE : GLenum =   0x8455;
pub const GL_FOG_COORDINATE_ARRAY_POINTER : GLenum =  0x8456;
pub const GL_FOG_COORDINATE_ARRAY : GLenum =          0x8457;

pub const GL_COLOR_SUM : GLenum =                     0x8458;
pub const GL_CURRENT_SECONDARY_COLOR : GLenum =       0x8459;
pub const GL_SECONDARY_COLOR_ARRAY_SIZE : GLenum =    0x845A;
pub const GL_SECONDARY_COLOR_ARRAY_TYPE : GLenum =    0x845B;
pub const GL_SECONDARY_COLOR_ARRAY_STRIDE : GLenum =  0x845C;
pub const GL_SECONDARY_COLOR_ARRAY_POINTER : GLenum = 0x845D;
pub const GL_SECONDARY_COLOR_ARRAY : GLenum =         0x845E;

pub const GL_POINT_SIZE_MIN : GLenum =                0x8126;
pub const GL_POINT_SIZE_MAX : GLenum =                0x8127;
pub const GL_POINT_FADE_THRESHOLD_SIZE : GLenum =     0x8128;
pub const GL_POINT_DISTANCE_ATTENUATION : GLenum =    0x8129;

pub const GL_BLEND_DST_RGB : GLenum =                 0x80C8;
pub const GL_BLEND_SRC_RGB : GLenum =                 0x80C9;
pub const GL_BLEND_DST_ALPHA : GLenum =               0x80CA;
pub const GL_BLEND_SRC_ALPHA : GLenum =               0x80CB;

pub const GL_GENERATE_MIPMAP : GLenum =               0x8191;
pub const GL_GENERATE_MIPMAP_HINT : GLenum =          0x8192;

pub const GL_INCR_WRAP : GLenum =                     0x8507;
pub const GL_DECR_WRAP : GLenum =                     0x8508;

pub const GL_MIRRORED_REPEAT : GLenum =               0x8370;

pub const GL_MAX_TEXTURE_LOD_BIAS : GLenum =          0x84FD;
pub const GL_TEXTURE_FILTER_CONTROL : GLenum =        0x8500;
pub const GL_TEXTURE_LOD_BIAS : GLenum =              0x8501;

/* vertex_buffer_object */
pub const GL_ARRAY_BUFFER : GLenum =                               0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER : GLenum =                       0x8893;
pub const GL_ARRAY_BUFFER_BINDING : GLenum =                       0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING : GLenum =               0x8895;
pub const GL_VERTEX_ARRAY_BUFFER_BINDING : GLenum =                0x8896;
pub const GL_NORMAL_ARRAY_BUFFER_BINDING : GLenum =                0x8897;
pub const GL_COLOR_ARRAY_BUFFER_BINDING : GLenum =                 0x8898;
pub const GL_INDEX_ARRAY_BUFFER_BINDING : GLenum =                 0x8899;
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING : GLenum =        0x889A;
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING : GLenum =             0x889B;
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING : GLenum =       0x889C;
pub const GL_FOG_COORD_ARRAY_BUFFER_BINDING : GLenum =             0x889D;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING : GLenum =                0x889E;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING : GLenum =         0x889F;
pub const GL_STREAM_DRAW : GLenum =                                0x88E0;
pub const GL_STREAM_READ : GLenum =                                0x88E1;
pub const GL_STREAM_COPY : GLenum =                                0x88E2;
pub const GL_STATIC_DRAW : GLenum =                                0x88E4;
pub const GL_STATIC_READ : GLenum =                                0x88E5;
pub const GL_STATIC_COPY : GLenum =                                0x88E6;
pub const GL_DYNAMIC_DRAW : GLenum =                               0x88E8;
pub const GL_DYNAMIC_READ : GLenum =                               0x88E9;
pub const GL_DYNAMIC_COPY : GLenum =                               0x88EA;
pub const GL_READ_ONLY : GLenum =                                  0x88B8;
pub const GL_WRITE_ONLY : GLenum =                                 0x88B9;
pub const GL_READ_WRITE : GLenum =                                 0x88BA;
pub const GL_BUFFER_SIZE : GLenum =                                0x8764;
pub const GL_BUFFER_USAGE : GLenum =                               0x8765;
pub const GL_BUFFER_ACCESS : GLenum =                              0x88BB;
pub const GL_BUFFER_MAPPED : GLenum =                              0x88BC;
pub const GL_BUFFER_MAP_POINTER : GLenum =                         0x88BD;
/* Obsolete */
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING : GLenum =        0x889D;

/* OpenGL 2.0 */
pub const GL_CURRENT_PROGRAM : GLenum =               0x8B8D;
pub const GL_SHADER_TYPE : GLenum =                   0x8B4F;
pub const GL_DELETE_STATUS : GLenum =                 0x8B80;
pub const GL_COMPILE_STATUS : GLenum =                0x8B81;
pub const GL_LINK_STATUS : GLenum =                   0x8B82;
pub const GL_VALIDATE_STATUS : GLenum =               0x8B83;
pub const GL_INFO_LOG_LENGTH : GLenum =               0x8B84;
pub const GL_ATTACHED_SHADERS : GLenum =              0x8B85;
pub const GL_ACTIVE_UNIFORMS : GLenum =               0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH : GLenum =     0x8B87;
pub const GL_SHADER_SOURCE_LENGTH : GLenum =          0x8B88;
pub const GL_FLOAT_VEC2 : GLenum =                    0x8B50;
pub const GL_FLOAT_VEC3 : GLenum =                    0x8B51;
pub const GL_FLOAT_VEC4 : GLenum =                    0x8B52;
pub const GL_INT_VEC2 : GLenum =                      0x8B53;
pub const GL_INT_VEC3 : GLenum =                      0x8B54;
pub const GL_INT_VEC4 : GLenum =                      0x8B55;
pub const GL_BOOL : GLenum =                          0x8B56;
pub const GL_BOOL_VEC2 : GLenum =                     0x8B57;
pub const GL_BOOL_VEC3 : GLenum =                     0x8B58;
pub const GL_BOOL_VEC4 : GLenum =                     0x8B59;
pub const GL_FLOAT_MAT2 : GLenum =                    0x8B5A;
pub const GL_FLOAT_MAT3 : GLenum =                    0x8B5B;
pub const GL_FLOAT_MAT4 : GLenum =                    0x8B5C;
pub const GL_SAMPLER_1D : GLenum =                    0x8B5D;
pub const GL_SAMPLER_2D : GLenum =                    0x8B5E;
pub const GL_SAMPLER_3D : GLenum =                    0x8B5F;
pub const GL_SAMPLER_CUBE : GLenum =                  0x8B60;
pub const GL_SAMPLER_1D_SHADOW : GLenum =             0x8B61;
pub const GL_SAMPLER_2D_SHADOW : GLenum =             0x8B62;
pub const GL_SHADING_LANGUAGE_VERSION : GLenum =      0x8B8C;
pub const GL_VERTEX_SHADER : GLenum =                 0x8B31;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS : GLenum = 0x8B4A;
pub const GL_MAX_VARYING_FLOATS : GLenum =            0x8B4B;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS : c_int =0x8B4C;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS : c_int =0x8B4D;
pub const GL_ACTIVE_ATTRIBUTES : GLenum =             0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH : GLenum =   0x8B8A;
pub const GL_FRAGMENT_SHADER : GLenum =               0x8B30;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS : c_int =0x8B49;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT : c_int =0x8B8B;
pub const GL_MAX_VERTEX_ATTRIBS : GLenum =            0x8869;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED : GLenum =   0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE : GLenum =      0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE : GLenum =    0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE : GLenum =      0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED : c_int =0x886A;
pub const GL_CURRENT_VERTEX_ATTRIB : GLenum =         0x8626;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER : GLenum =   0x8645;
pub const GL_VERTEX_PROGRAM_POINT_SIZE : GLenum =     0x8642;
pub const GL_VERTEX_PROGRAM_TWO_SIDE : GLenum =       0x8643;
pub const GL_MAX_TEXTURE_COORDS : GLenum =            0x8871;
pub const GL_MAX_TEXTURE_IMAGE_UNITS : GLenum =       0x8872;
pub const GL_MAX_DRAW_BUFFERS : GLenum =              0x8824;
pub const GL_DRAW_BUFFER0 : GLenum =                  0x8825;
pub const GL_DRAW_BUFFER1 : GLenum =                  0x8826;
pub const GL_DRAW_BUFFER2 : GLenum =                  0x8827;
pub const GL_DRAW_BUFFER3 : GLenum =                  0x8828;
pub const GL_DRAW_BUFFER4 : GLenum =                  0x8829;
pub const GL_DRAW_BUFFER5 : GLenum =                  0x882A;
pub const GL_DRAW_BUFFER6 : GLenum =                  0x882B;
pub const GL_DRAW_BUFFER7 : GLenum =                  0x882C;
pub const GL_DRAW_BUFFER8 : GLenum =                  0x882D;
pub const GL_DRAW_BUFFER9 : GLenum =                  0x882E;
pub const GL_DRAW_BUFFER10 : GLenum =                 0x882F;
pub const GL_DRAW_BUFFER11 : GLenum =                 0x8830;
pub const GL_DRAW_BUFFER12 : GLenum =                 0x8831;
pub const GL_DRAW_BUFFER13 : GLenum =                 0x8832;
pub const GL_DRAW_BUFFER14 : GLenum =                 0x8833;
pub const GL_DRAW_BUFFER15 : GLenum =                 0x8834;
pub const GL_POINT_SPRITE : GLenum =                  0x8861;
pub const GL_COORD_REPLACE : GLenum =                 0x8862;
pub const GL_POINT_SPRITE_COORD_ORIGIN : GLenum =     0x8CA0;
pub const GL_LOWER_LEFT : GLenum =                    0x8CA1;
pub const GL_UPPER_LEFT : GLenum =                    0x8CA2;
pub const GL_STENCIL_BACK_FUNC : GLenum =             0x8800;
pub const GL_STENCIL_BACK_VALUE_MASK : GLenum =       0x8CA4;
pub const GL_STENCIL_BACK_REF : GLenum =              0x8CA3;
pub const GL_STENCIL_BACK_FAIL : GLenum =             0x8801;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL : GLenum =  0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS : GLenum =  0x8803;
pub const GL_STENCIL_BACK_WRITEMASK : GLenum =        0x8CA5;

/* OpenGL 2.1 */
pub const GL_CURRENT_RASTER_SECONDARY_COLOR : c_int =0x845F;
pub const GL_PIXEL_PACK_BUFFER : GLenum =             0x88EB;
pub const GL_PIXEL_UNPACK_BUFFER : GLenum =           0x88EC;
pub const GL_PIXEL_PACK_BUFFER_BINDING : GLenum =     0x88ED;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING : GLenum =   0x88EF;
pub const GL_FLOAT_MAT2x3 : GLenum =                  0x8B65;
pub const GL_FLOAT_MAT2x4 : GLenum =                  0x8B66;
pub const GL_FLOAT_MAT3x2 : GLenum =                  0x8B67;
pub const GL_FLOAT_MAT3x4 : GLenum =                  0x8B68;
pub const GL_FLOAT_MAT4x2 : GLenum =                  0x8B69;
pub const GL_FLOAT_MAT4x3 : GLenum =                  0x8B6A;
pub const GL_SRGB : GLenum =                          0x8C40;
pub const GL_SRGB8 : GLenum =                         0x8C41;
pub const GL_SRGB_ALPHA : GLenum =                    0x8C42;
pub const GL_SRGB8_ALPHA8 : GLenum =                  0x8C43;
pub const GL_SLUMINANCE_ALPHA : GLenum =              0x8C44;
pub const GL_SLUMINANCE8_ALPHA8 : GLenum =            0x8C45;
pub const GL_SLUMINANCE : GLenum =                    0x8C46;
pub const GL_SLUMINANCE8 : GLenum =                   0x8C47;
pub const GL_COMPRESSED_SRGB : GLenum =               0x8C48;
pub const GL_COMPRESSED_SRGB_ALPHA : GLenum =         0x8C49;
pub const GL_COMPRESSED_SLUMINANCE : GLenum =         0x8C4A;
pub const GL_COMPRESSED_SLUMINANCE_ALPHA : GLenum =   0x8C4B;

/*************************************************************/

pub extern "C" {

    pub fn glAccum(op: GLenum, value: GLfloat);
    pub fn glAlphaFunc(func: GLenum, _ref: GLclampf);
    pub fn glAreTexturesResident(n: GLsizei, textures: *GLuint /*const*/, residences: *GLboolean) -> GLboolean ;
    pub fn glArrayElement(i: GLint);
    pub fn glBegin(mode: GLenum);
    pub fn glBindTexture(target: GLenum, texture: GLuint);
    pub fn glBitmap(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *GLubyte /*const*/);
    pub fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    pub fn glBlendEquation(mode: GLenum);
    pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    pub fn glCallList(list: GLuint);
    pub fn glCallLists(n: GLsizei, _type: GLenum, lists: *GLvoid /*const*/);
    pub fn glClear(mask: GLbitfield);
    pub fn glClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    pub fn glClearDepth(depth: GLclampd);
    pub fn glClearIndex(c: GLfloat);
    pub fn glClearStencil(s: GLint);
    pub fn glClipPlane(plane: GLenum, equation: *GLdouble /*const*/);
    pub fn glColor3b(red: GLbyte, green: GLbyte, blue: GLbyte);
    pub fn glColor3bv(v: *GLbyte /*const*/);
    pub fn glColor3d(red: GLdouble, green: GLdouble, blue: GLdouble);
    pub fn glColor3dv(v: *GLdouble /*const*/);
    pub fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
    pub fn glColor3fv(v: *GLfloat /*const*/);
    pub fn glColor3i(red: GLint, green: GLint, blue: GLint);
    pub fn glColor3iv(v: *GLint /*const*/);
    pub fn glColor3s(red: GLshort, green: GLshort, blue: GLshort);
    pub fn glColor3sv(v: *GLshort /*const*/);
    pub fn glColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte);
    pub fn glColor3ubv(v: *GLubyte /*const*/);
    pub fn glColor3ui(red: GLuint, green: GLuint, blue: GLuint);
    pub fn glColor3uiv(v: *GLuint /*const*/);
    pub fn glColor3us(red: GLushort, green: GLushort, blue: GLushort);
    pub fn glColor3usv(v: *GLushort /*const*/);
    pub fn glColor4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte);
    pub fn glColor4bv(v: *GLbyte /*const*/);
    pub fn glColor4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble);
    pub fn glColor4dv(v: *GLdouble /*const*/);
    pub fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    pub fn glColor4fv(v: *GLfloat /*const*/);
    pub fn glColor4i(red: GLint, green: GLint, blue: GLint, alpha: GLint);
    pub fn glColor4iv(v: *GLint /*const*/);
    pub fn glColor4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort);
    pub fn glColor4sv(v: *GLshort /*const*/);
    pub fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);
    pub fn glColor4ubv(v: *GLubyte /*const*/);
    pub fn glColor4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);
    pub fn glColor4uiv(v: *GLuint /*const*/);
    pub fn glColor4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort);
    pub fn glColor4usv(v: *GLushort /*const*/);
    pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
    pub fn glColorMaterial(face: GLenum, mode: GLenum);
    pub fn glColorPointer(size: GLint, _type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glColorSubTable(target: GLenum, start: GLsizei, count: GLsizei, format: GLenum, _type: GLenum, data: *GLvoid /*const*/);
    pub fn glColorTable(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, _type: GLenum, table: *GLvoid /*const*/);
    pub fn glColorTableParameterfv(target: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glColorTableParameteriv(target: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glConvolutionFilter1D(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, _type: GLenum, image: *GLvoid /*const*/);
    pub fn glConvolutionFilter2D(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, image: *GLvoid /*const*/);
    pub fn glConvolutionParameterf(target: GLenum, pname: GLenum, params: GLfloat);
    pub fn glConvolutionParameterfv(target: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glConvolutionParameteri(target: GLenum, pname: GLenum, params: GLint);
    pub fn glConvolutionParameteriv(target: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glCopyColorSubTable(target: GLenum, start: GLsizei, x: GLint, y: GLint, width: GLsizei);
    pub fn glCopyColorTable(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);
    pub fn glCopyConvolutionFilter1D(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);
    pub fn glCopyConvolutionFilter2D(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glCopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, _type: GLenum);
    pub fn glCopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
    pub fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
    pub fn glCopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
    pub fn glCopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glCopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glCullFace(mode: GLenum);
    pub fn glDeleteLists(list: GLuint, range: GLsizei);
    pub fn glDeleteTextures(n: GLsizei, textures: *GLuint /*const*/);
    pub fn glDepthFunc(func: GLenum);
    pub fn glDepthMask(flag: GLboolean);
    pub fn glDepthRange(zNear: GLclampd, zFar: GLclampd);
    pub fn glDisable(cap: GLenum);
    pub fn glDisableClientState(array: GLenum);
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    pub fn glDrawBuffer(mode: GLenum);
    pub fn glDrawElements(mode: GLenum, count: GLsizei, _type: GLenum, indices: *GLvoid /*const*/);
    pub fn glDrawPixels(width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glDrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, _type: GLenum, indices: *GLvoid /*const*/);
    pub fn glEdgeFlag(flag: GLboolean);
    pub fn glEdgeFlagPointer(stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glEdgeFlagv(flag: *GLboolean /*const*/);
    pub fn glEnable(cap: GLenum);
    pub fn glEnableClientState(array: GLenum);
    pub fn glEnd();
    pub fn glEndList();
    pub fn glEvalCoord1d(u: GLdouble);
    pub fn glEvalCoord1dv(u: *GLdouble /*const*/);
    pub fn glEvalCoord1f(u: GLfloat);
    pub fn glEvalCoord1fv(u: *GLfloat /*const*/);
    pub fn glEvalCoord2d(u: GLdouble, v: GLdouble);
    pub fn glEvalCoord2dv(u: *GLdouble /*const*/);
    pub fn glEvalCoord2f(u: GLfloat, v: GLfloat);
    pub fn glEvalCoord2fv(u: *GLfloat /*const*/);
    pub fn glEvalMesh1(mode: GLenum, i1: GLint, i2: GLint);
    pub fn glEvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint);
    pub fn glEvalPoint1(i: GLint);
    pub fn glEvalPoint2(i: GLint, j: GLint);
    pub fn glFeedbackBuffer(size: GLsizei, _type: GLenum, buffer: *GLfloat);
    pub fn glFinish();
    pub fn glFlush();
    pub fn glFogf(pname: GLenum, param: GLfloat);
    pub fn glFogfv(pname: GLenum, params: *GLfloat /*const*/);
    pub fn glFogi(pname: GLenum, param: GLint);
    pub fn glFogiv(pname: GLenum, params: *GLint /*const*/);
    pub fn glFrontFace(mode: GLenum);
    pub fn glFrustum(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
    pub fn glGenLists(range: GLsizei) -> GLuint ;
    pub fn glGenTextures(n: GLsizei, textures: *GLuint);
    pub fn glGetBooleanv(pname: GLenum, params: *GLboolean);
    pub fn glGetClipPlane(plane: GLenum, equation: *GLdouble);
    pub fn glGetColorTable(target: GLenum, format: GLenum, _type: GLenum, table: *GLvoid);
    pub fn glGetColorTableParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetColorTableParameteriv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetConvolutionFilter(target: GLenum, format: GLenum, _type: GLenum, image: *GLvoid);
    pub fn glGetConvolutionParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetConvolutionParameteriv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetDoublev(pname: GLenum, params: *GLdouble);
    pub fn glGetError() -> GLenum ;
    pub fn glGetFloatv(pname: GLenum, params: *GLfloat);
    pub fn glGetHistogram(target: GLenum, reset: GLboolean, format: GLenum, _type: GLenum, values: *GLvoid);
    pub fn glGetHistogramParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetHistogramParameteriv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetIntegerv(pname: GLenum, params: *GLint);
    pub fn glGetLightfv(light: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetLightiv(light: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetMapdv(target: GLenum, query: GLenum, v: *GLdouble);
    pub fn glGetMapfv(target: GLenum, query: GLenum, v: *GLfloat);
    pub fn glGetMapiv(target: GLenum, query: GLenum, v: *GLint);
    pub fn glGetMaterialfv(face: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetMaterialiv(face: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetMinmax(target: GLenum, reset: GLboolean, format: GLenum, _type: GLenum, values: *GLvoid);
    pub fn glGetMinmaxParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetMinmaxParameteriv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetPixelMapfv(map: GLenum, values: *GLfloat);
    pub fn glGetPixelMapuiv(map: GLenum, values: *GLuint);
    pub fn glGetPixelMapusv(map: GLenum, values: *GLushort);
    pub fn glGetPointerv(pname: GLenum, params: **GLvoid);
    pub fn glGetPolygonStipple(mask: *GLubyte);
    pub fn glGetSeparableFilter(target: GLenum, format: GLenum, _type: GLenum, row: *GLvoid, column: *GLvoid, span: *GLvoid);
    pub fn glGetString(name: GLenum) -> /*const*/ *GLubyte;
    pub fn glGetTexEnvfv(target: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetTexEnviv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetTexGendv(coord: GLenum, pname: GLenum, params: *GLdouble);
    pub fn glGetTexGenfv(coord: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetTexGeniv(coord: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetTexImage(target: GLenum, level: GLint, format: GLenum, _type: GLenum, pixels: *GLvoid);
    pub fn glGetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *GLfloat);
    pub fn glGetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *GLint);
    pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);
    pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glHint(target: GLenum, mode: GLenum);
    pub fn glHistogram(target: GLenum, width: GLsizei, internalformat: GLenum, sink: GLboolean);
    pub fn glIndexMask(mask: GLuint);
    pub fn glIndexPointer(_type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glIndexd(c: GLdouble);
    pub fn glIndexdv(c: *GLdouble /*const*/);
    pub fn glIndexf(c: GLfloat);
    pub fn glIndexfv(c: *GLfloat /*const*/);
    pub fn glIndexi(c: GLint);
    pub fn glIndexiv(c: *GLint /*const*/);
    pub fn glIndexs(c: GLshort);
    pub fn glIndexsv(c: *GLshort /*const*/);
    pub fn glIndexub(c: GLubyte);
    pub fn glIndexubv(c: *GLubyte /*const*/);
    pub fn glInitNames();
    pub fn glInterleavedArrays(format: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glIsEnabled(cap: GLenum) -> GLboolean ;
    pub fn glIsList(list: GLuint) -> GLboolean ;
    pub fn glIsTexture(texture: GLuint) -> GLboolean ;
    pub fn glLightModelf(pname: GLenum, param: GLfloat);
    pub fn glLightModelfv(pname: GLenum, params: *GLfloat /*const*/);
    pub fn glLightModeli(pname: GLenum, param: GLint);
    pub fn glLightModeliv(pname: GLenum, params: *GLint /*const*/);
    pub fn glLightf(light: GLenum, pname: GLenum, param: GLfloat);
    pub fn glLightfv(light: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glLighti(light: GLenum, pname: GLenum, param: GLint);
    pub fn glLightiv(light: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glLineStipple(factor: GLint, pattern: GLushort);
    pub fn glLineWidth(width: GLfloat);
    pub fn glListBase(base: GLuint);
    pub fn glLoadIdentity();
    pub fn glLoadMatrixd(m: *GLdouble /*const*/);
    pub fn glLoadMatrixf(m: *GLfloat /*const*/);
    pub fn glLoadName(name: GLuint);
    pub fn glLogicOp(opcode: GLenum);
    pub fn glMap1d(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *GLdouble /*const*/);
    pub fn glMap1f(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *GLfloat /*const*/);
    pub fn glMap2d(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *GLdouble /*const*/);
    pub fn glMap2f(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *GLfloat /*const*/);
    pub fn glMapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble);
    pub fn glMapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat);
    pub fn glMapGrid2d(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble);
    pub fn glMapGrid2f(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat);
    pub fn glMaterialf(face: GLenum, pname: GLenum, param: GLfloat);
    pub fn glMaterialfv(face: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glMateriali(face: GLenum, pname: GLenum, param: GLint);
    pub fn glMaterialiv(face: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glMatrixMode(mode: GLenum);
    pub fn glMinmax(target: GLenum, internalformat: GLenum, sink: GLboolean);
    pub fn glMultMatrixd(m: *GLdouble /*const*/);
    pub fn glMultMatrixf(m: *GLfloat /*const*/);
    pub fn glNewList(list: GLuint, mode: GLenum);
    pub fn glNormal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte);
    pub fn glNormal3bv(v: *GLbyte /*const*/);
    pub fn glNormal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble);
    pub fn glNormal3dv(v: *GLdouble /*const*/);
    pub fn glNormal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat);
    pub fn glNormal3fv(v: *GLfloat /*const*/);
    pub fn glNormal3i(nx: GLint, ny: GLint, nz: GLint);
    pub fn glNormal3iv(v: *GLint /*const*/);
    pub fn glNormal3s(nx: GLshort, ny: GLshort, nz: GLshort);
    pub fn glNormal3sv(v: *GLshort /*const*/);
    pub fn glNormalPointer(_type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
    pub fn glPassThrough(token: GLfloat);
    pub fn glPixelMapfv(map: GLenum, mapsize: GLint, values: *GLfloat /*const*/);
    pub fn glPixelMapuiv(map: GLenum, mapsize: GLint, values: *GLuint /*const*/);
    pub fn glPixelMapusv(map: GLenum, mapsize: GLint, values: *GLushort /*const*/);
    pub fn glPixelStoref(pname: GLenum, param: GLfloat);
    pub fn glPixelStorei(pname: GLenum, param: GLint);
    pub fn glPixelTransferf(pname: GLenum, param: GLfloat);
    pub fn glPixelTransferi(pname: GLenum, param: GLint);
    pub fn glPixelZoom(xfactor: GLfloat, yfactor: GLfloat);
    pub fn glPointSize(size: GLfloat);
    pub fn glPolygonMode(face: GLenum, mode: GLenum);
    pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
    pub fn glPolygonStipple(mask: *GLubyte /*const*/);
    pub fn glPopAttrib();
    pub fn glPopClientAttrib();
    pub fn glPopMatrix();
    pub fn glPopName();
    pub fn glPrioritizeTextures(n: GLsizei, textures: *GLuint /*const*/, priorities: *GLclampf /*const*/);
    pub fn glPushAttrib(mask: GLbitfield);
    pub fn glPushClientAttrib(mask: GLbitfield);
    pub fn glPushMatrix();
    pub fn glPushName(name: GLuint);
    pub fn glRasterPos2d(x: GLdouble, y: GLdouble);
    pub fn glRasterPos2dv(v: *GLdouble /*const*/);
    pub fn glRasterPos2f(x: GLfloat, y: GLfloat);
    pub fn glRasterPos2fv(v: *GLfloat /*const*/);
    pub fn glRasterPos2i(x: GLint, y: GLint);
    pub fn glRasterPos2iv(v: *GLint /*const*/);
    pub fn glRasterPos2s(x: GLshort, y: GLshort);
    pub fn glRasterPos2sv(v: *GLshort /*const*/);
    pub fn glRasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glRasterPos3dv(v: *GLdouble /*const*/);
    pub fn glRasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glRasterPos3fv(v: *GLfloat /*const*/);
    pub fn glRasterPos3i(x: GLint, y: GLint, z: GLint);
    pub fn glRasterPos3iv(v: *GLint /*const*/);
    pub fn glRasterPos3s(x: GLshort, y: GLshort, z: GLshort);
    pub fn glRasterPos3sv(v: *GLshort /*const*/);
    pub fn glRasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
    pub fn glRasterPos4dv(v: *GLdouble /*const*/);
    pub fn glRasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
    pub fn glRasterPos4fv(v: *GLfloat /*const*/);
    pub fn glRasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint);
    pub fn glRasterPos4iv(v: *GLint /*const*/);
    pub fn glRasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort);
    pub fn glRasterPos4sv(v: *GLshort /*const*/);
    pub fn glReadBuffer(mode: GLenum);
    pub fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid);
    pub fn glRectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble);
    pub fn glRectdv(v1: *GLdouble /*const*/, v2: *GLdouble /*const*/);
    pub fn glRectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat);
    pub fn glRectfv(v1: *GLfloat /*const*/, v2: *GLfloat /*const*/);
    pub fn glRecti(x1: GLint, y1: GLint, x2: GLint, y2: GLint);
    pub fn glRectiv(v1: *GLint /*const*/, v2: *GLint /*const*/);
    pub fn glRects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort);
    pub fn glRectsv(v1: *GLshort /*const*/, v2: *GLshort /*const*/);
    pub fn glRenderMode(mode: GLenum) -> GLint ;
    pub fn glResetHistogram(target: GLenum);
    pub fn glResetMinmax(target: GLenum);
    pub fn glRotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glScaled(x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glSelectBuffer(size: GLsizei, buffer: *GLuint);
    pub fn glSeparableFilter2D(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, row: *GLvoid /*const*/, column: *GLvoid /*const*/);
    pub fn glShadeModel(mode: GLenum);
    pub fn glStencilFunc(func: GLenum, _ref: GLint, mask: GLuint);
    pub fn glStencilMask(mask: GLuint);
    pub fn glStencilOp(_fail: GLenum, zfail: GLenum, zpass: GLenum);
    pub fn glTexCoord1d(s: GLdouble);
    pub fn glTexCoord1dv(v: *GLdouble /*const*/);
    pub fn glTexCoord1f(s: GLfloat);
    pub fn glTexCoord1fv(v: *GLfloat /*const*/);
    pub fn glTexCoord1i(s: GLint);
    pub fn glTexCoord1iv(v: *GLint /*const*/);
    pub fn glTexCoord1s(s: GLshort);
    pub fn glTexCoord1sv(v: *GLshort /*const*/);
    pub fn glTexCoord2d(s: GLdouble, t: GLdouble);
    pub fn glTexCoord2dv(v: *GLdouble /*const*/);
    pub fn glTexCoord2f(s: GLfloat, t: GLfloat);
    pub fn glTexCoord2fv(v: *GLfloat /*const*/);
    pub fn glTexCoord2i(s: GLint, t: GLint);
    pub fn glTexCoord2iv(v: *GLint /*const*/);
    pub fn glTexCoord2s(s: GLshort, t: GLshort);
    pub fn glTexCoord2sv(v: *GLshort /*const*/);
    pub fn glTexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble);
    pub fn glTexCoord3dv(v: *GLdouble /*const*/);
    pub fn glTexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat);
    pub fn glTexCoord3fv(v: *GLfloat /*const*/);
    pub fn glTexCoord3i(s: GLint, t: GLint, r: GLint);
    pub fn glTexCoord3iv(v: *GLint /*const*/);
    pub fn glTexCoord3s(s: GLshort, t: GLshort, r: GLshort);
    pub fn glTexCoord3sv(v: *GLshort /*const*/);
    pub fn glTexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
    pub fn glTexCoord4dv(v: *GLdouble /*const*/);
    pub fn glTexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
    pub fn glTexCoord4fv(v: *GLfloat /*const*/);
    pub fn glTexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint);
    pub fn glTexCoord4iv(v: *GLint /*const*/);
    pub fn glTexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort);
    pub fn glTexCoord4sv(v: *GLshort /*const*/);
    pub fn glTexCoordPointer(size: GLint, _type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
    pub fn glTexEnvfv(target: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glTexEnvi(target: GLenum, pname: GLenum, param: GLint);
    pub fn glTexEnviv(target: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glTexGend(coord: GLenum, pname: GLenum, param: GLdouble);
    pub fn glTexGendv(coord: GLenum, pname: GLenum, params: *GLdouble /*const*/);
    pub fn glTexGenf(coord: GLenum, pname: GLenum, param: GLfloat);
    pub fn glTexGenfv(coord: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glTexGeni(coord: GLenum, pname: GLenum, param: GLint);
    pub fn glTexGeniv(coord: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glTexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glTexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
    pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat /*const*/);
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *GLint /*const*/);
    pub fn glTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid /*const*/);
    pub fn glTranslated(x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glVertex2d(x: GLdouble, y: GLdouble);
    pub fn glVertex2dv(v: *GLdouble /*const*/);
    pub fn glVertex2f(x: GLfloat, y: GLfloat);
    pub fn glVertex2fv(v: *GLfloat /*const*/);
    pub fn glVertex2i(x: GLint, y: GLint);
    pub fn glVertex2iv(v: *GLint /*const*/);
    pub fn glVertex2s(x: GLshort, y: GLshort);
    pub fn glVertex2sv(v: *GLshort /*const*/);
    pub fn glVertex3d(x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glVertex3dv(v: *GLdouble /*const*/);
    pub fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glVertex3fv(v: *GLfloat /*const*/);
    pub fn glVertex3i(x: GLint, y: GLint, z: GLint);
    pub fn glVertex3iv(v: *GLint /*const*/);
    pub fn glVertex3s(x: GLshort, y: GLshort, z: GLshort);
    pub fn glVertex3sv(v: *GLshort /*const*/);
    pub fn glVertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
    pub fn glVertex4dv(v: *GLdouble /*const*/);
    pub fn glVertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
    pub fn glVertex4fv(v: *GLfloat /*const*/);
    pub fn glVertex4i(x: GLint, y: GLint, z: GLint, w: GLint);
    pub fn glVertex4iv(v: *GLint /*const*/);
    pub fn glVertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort);
    pub fn glVertex4sv(v: *GLshort /*const*/);
    pub fn glVertexPointer(size: GLint, _type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glSampleCoverage(value: GLclampf, invert: GLboolean);
    pub fn glLoadTransposeMatrixf(m: *GLfloat /*const*/);
    pub fn glLoadTransposeMatrixd(m: *GLdouble /*const*/);
    pub fn glMultTransposeMatrixf(m: *GLfloat /*const*/);
    pub fn glMultTransposeMatrixd(m: *GLdouble /*const*/);
    pub fn glCompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid /*const*/);
    pub fn glCompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid /*const*/);
    pub fn glCompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid /*const*/);
    pub fn glCompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid /*const*/);
    pub fn glCompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid /*const*/);
    pub fn glCompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid /*const*/);
    pub fn glGetCompressedTexImage(target: GLenum, lod: GLint, img: *GLvoid);
    pub fn glActiveTexture(texture: GLenum);
    pub fn glClientActiveTexture(texture: GLenum);
    pub fn glMultiTexCoord1d(target: GLenum, s: GLdouble);
    pub fn glMultiTexCoord1dv(target: GLenum, v: *GLdouble /*const*/);
    pub fn glMultiTexCoord1f(target: GLenum, s: GLfloat);
    pub fn glMultiTexCoord1fv(target: GLenum, v: *GLfloat /*const*/);
    pub fn glMultiTexCoord1i(target: GLenum, s: GLint);
    pub fn glMultiTexCoord1iv(target: GLenum, v: *GLint /*const*/);
    pub fn glMultiTexCoord1s(target: GLenum, s: GLshort);
    pub fn glMultiTexCoord1sv(target: GLenum, v: *GLshort /*const*/);
    pub fn glMultiTexCoord2d(target: GLenum, s: GLdouble, t: GLdouble);
    pub fn glMultiTexCoord2dv(target: GLenum, v: *GLdouble /*const*/);
    pub fn glMultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat);
    pub fn glMultiTexCoord2fv(target: GLenum, v: *GLfloat /*const*/);
    pub fn glMultiTexCoord2i(target: GLenum, s: GLint, t: GLint);
    pub fn glMultiTexCoord2iv(target: GLenum, v: *GLint /*const*/);
    pub fn glMultiTexCoord2s(target: GLenum, s: GLshort, t: GLshort);
    pub fn glMultiTexCoord2sv(target: GLenum, v: *GLshort /*const*/);
    pub fn glMultiTexCoord3d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);
    pub fn glMultiTexCoord3dv(target: GLenum, v: *GLdouble /*const*/);
    pub fn glMultiTexCoord3f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);
    pub fn glMultiTexCoord3fv(target: GLenum, v: *GLfloat /*const*/);
    pub fn glMultiTexCoord3i(target: GLenum, s: GLint, t: GLint, r: GLint);
    pub fn glMultiTexCoord3iv(target: GLenum, v: *GLint /*const*/);
    pub fn glMultiTexCoord3s(target: GLenum, s: GLshort, t: GLshort, r: GLshort);
    pub fn glMultiTexCoord3sv(target: GLenum, v: *GLshort /*const*/);
    pub fn glMultiTexCoord4d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
    pub fn glMultiTexCoord4dv(target: GLenum, v: *GLdouble /*const*/);
    pub fn glMultiTexCoord4f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
    pub fn glMultiTexCoord4fv(target: GLenum, v: *GLfloat /*const*/);
    pub fn glMultiTexCoord4i(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);
    pub fn glMultiTexCoord4iv(target: GLenum, v: *GLint /*const*/);
    pub fn glMultiTexCoord4s(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);
    pub fn glMultiTexCoord4sv(target: GLenum, v: *GLshort /*const*/);
    pub fn glFogCoordf(coord: GLfloat);
    pub fn glFogCoordfv(coord: *GLfloat /*const*/);
    pub fn glFogCoordd(coord: GLdouble);
    pub fn glFogCoorddv(coord: *GLdouble /*const*/);
    pub fn glFogCoordPointer(_type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glSecondaryColor3b(red: GLbyte, green: GLbyte, blue: GLbyte);
    pub fn glSecondaryColor3bv(v: *GLbyte /*const*/);
    pub fn glSecondaryColor3d(red: GLdouble, green: GLdouble, blue: GLdouble);
    pub fn glSecondaryColor3dv(v: *GLdouble /*const*/);
    pub fn glSecondaryColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
    pub fn glSecondaryColor3fv(v: *GLfloat /*const*/);
    pub fn glSecondaryColor3i(red: GLint, green: GLint, blue: GLint);
    pub fn glSecondaryColor3iv(v: *GLint /*const*/);
    pub fn glSecondaryColor3s(red: GLshort, green: GLshort, blue: GLshort);
    pub fn glSecondaryColor3sv(v: *GLshort /*const*/);
    pub fn glSecondaryColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte);
    pub fn glSecondaryColor3ubv(v: *GLubyte /*const*/);
    pub fn glSecondaryColor3ui(red: GLuint, green: GLuint, blue: GLuint);
    pub fn glSecondaryColor3uiv(v: *GLuint /*const*/);
    pub fn glSecondaryColor3us(red: GLushort, green: GLushort, blue: GLushort);
    pub fn glSecondaryColor3usv(v: *GLushort /*const*/);
    pub fn glSecondaryColorPointer(size: GLint, _type: GLenum, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glPointParameterf(pname: GLenum, param: GLfloat);
    pub fn glPointParameterfv(pname: GLenum, params: *GLfloat /*const*/);
    pub fn glPointParameteri(pname: GLenum, param: GLint);
    pub fn glPointParameteriv(pname: GLenum, params: *GLint /*const*/);
    pub fn glBlendFuncSeparate(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
    pub fn glMultiDrawArrays(mode: GLenum, first: *GLint /*const*/, count: *GLsizei /*const*/, primcount: GLsizei);
    pub fn glMultiDrawElements(mode: GLenum, count: *GLsizei /*const*/, _type: GLenum, indices: **GLvoid /*const*/, primcount: GLsizei);
    pub fn glWindowPos2d(x: GLdouble, y: GLdouble);
    pub fn glWindowPos2dv(v: *GLdouble /*const*/);
    pub fn glWindowPos2f(x: GLfloat, y: GLfloat);
    pub fn glWindowPos2fv(v: *GLfloat /*const*/);
    pub fn glWindowPos2i(x: GLint, y: GLint);
    pub fn glWindowPos2iv(v: *GLint /*const*/);
    pub fn glWindowPos2s(x: GLshort, y: GLshort);
    pub fn glWindowPos2sv(v: *GLshort /*const*/);
    pub fn glWindowPos3d(x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glWindowPos3dv(v: *GLdouble /*const*/);
    pub fn glWindowPos3f(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glWindowPos3fv(v: *GLfloat /*const*/);
    pub fn glWindowPos3i(x: GLint, y: GLint, z: GLint);
    pub fn glWindowPos3iv(v: *GLint /*const*/);
    pub fn glWindowPos3s(x: GLshort, y: GLshort, z: GLshort);
    pub fn glWindowPos3sv(v: *GLshort /*const*/);
    pub fn glGenQueries(n: GLsizei, ids: *GLuint);
    pub fn glDeleteQueries(n: GLsizei, ids: *GLuint /*const*/);
    pub fn glIsQuery(id: GLuint) -> GLboolean ;
    pub fn glBeginQuery(target: GLenum, id: GLuint);
    pub fn glEndQuery(target: GLenum);
    pub fn glGetQueryiv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *GLint);
    pub fn glGetQueryObjectuiv(id: GLuint, pname: GLenum, params: *GLuint);
    pub fn glBindBuffer(target: GLenum, buffer: GLuint);
    pub fn glDeleteBuffers(n: GLsizei, buffers: *GLuint /*const*/);
    pub fn glGenBuffers(n: GLsizei, buffers: *GLuint);
    pub fn glIsBuffer(buffer: GLuint) -> GLboolean ;
    pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *GLvoid /*const*/, usage: GLenum);
    pub fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid /*const*/);
    pub fn glGetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid);
    pub fn glMapBuffer(target: GLenum, access: GLenum);
    pub fn glUnmapBuffer(target: GLenum) -> GLboolean ;
    pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *GLint);
    pub fn glGetBufferPointerv(target: GLenum, pname: GLenum, params: **GLvoid);
    pub fn glDrawBuffers(n: GLsizei, bufs: *GLenum /*const*/);
    pub fn glVertexAttrib1d(index: GLuint, x: GLdouble);
    pub fn glVertexAttrib1dv(index: GLuint, v: *GLdouble /*const*/);
    pub fn glVertexAttrib1f(index: GLuint, x: GLfloat);
    pub fn glVertexAttrib1fv(index: GLuint, v: *GLfloat /*const*/);
    pub fn glVertexAttrib1s(index: GLuint, x: GLshort);
    pub fn glVertexAttrib1sv(index: GLuint, v: *GLshort /*const*/);
    pub fn glVertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble);
    pub fn glVertexAttrib2dv(index: GLuint, v: *GLdouble /*const*/);
    pub fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
    pub fn glVertexAttrib2fv(index: GLuint, v: *GLfloat /*const*/);
    pub fn glVertexAttrib2s(index: GLuint, x: GLshort, y: GLshort);
    pub fn glVertexAttrib2sv(index: GLuint, v: *GLshort /*const*/);
    pub fn glVertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
    pub fn glVertexAttrib3dv(index: GLuint, v: *GLdouble /*const*/);
    pub fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glVertexAttrib3fv(index: GLuint, v: *GLfloat /*const*/);
    pub fn glVertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
    pub fn glVertexAttrib3sv(index: GLuint, v: *GLshort /*const*/);
    pub fn glVertexAttrib4Nbv(index: GLuint, v: *GLbyte /*const*/);
    pub fn glVertexAttrib4Niv(index: GLuint, v: *GLint /*const*/);
    pub fn glVertexAttrib4Nsv(index: GLuint, v: *GLshort /*const*/);
    pub fn glVertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
    pub fn glVertexAttrib4Nubv(index: GLuint, v: *GLubyte /*const*/);
    pub fn glVertexAttrib4Nuiv(index: GLuint, v: *GLuint /*const*/);
    pub fn glVertexAttrib4Nusv(index: GLuint, v: *GLushort /*const*/);
    pub fn glVertexAttrib4bv(index: GLuint, v: *GLbyte /*const*/);
    pub fn glVertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
    pub fn glVertexAttrib4dv(index: GLuint, v: *GLdouble /*const*/);
    pub fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
    pub fn glVertexAttrib4fv(index: GLuint, v: *GLfloat /*const*/);
    pub fn glVertexAttrib4iv(index: GLuint, v: *GLint /*const*/);
    pub fn glVertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
    pub fn glVertexAttrib4sv(index: GLuint, v: *GLshort /*const*/);
    pub fn glVertexAttrib4ubv(index: GLuint, v: *GLubyte /*const*/);
    pub fn glVertexAttrib4uiv(index: GLuint, v: *GLuint /*const*/);
    pub fn glVertexAttrib4usv(index: GLuint, v: *GLushort /*const*/);
    pub fn glVertexAttribPointer(index: GLuint, size: GLint, _type: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *GLvoid /*const*/);
    pub fn glEnableVertexAttribArray(index: GLuint);
    pub fn glDisableVertexAttribArray(index: GLuint);
    pub fn glGetVertexAttribdv(index: GLuint, pname: GLenum, params: *GLdouble);
    pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *GLfloat);
    pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *GLint);
    pub fn glGetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **GLvoid);
    pub fn glDeleteShader(shader: GLuint);
    pub fn glDetachShader(program: GLuint, shader: GLuint);
    pub fn glCreateShader(_type: GLenum) -> GLuint ;
    pub fn glShaderSource(shader: GLuint, count: GLsizei, string: **GLchar /*const*/, length: *GLint /*const*/);
    pub fn glCompileShader(shader: GLuint);
    pub fn glCreateProgram() -> GLuint ;
    pub fn glAttachShader(program: GLuint, shader: GLuint);
    pub fn glLinkProgram(program: GLuint);
    pub fn glUseProgram(program: GLuint);
    pub fn glDeleteProgram(program: GLuint);
    pub fn glValidateProgram(program: GLuint);
    pub fn glUniform1f(location: GLint, v0: GLfloat);
    pub fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
    pub fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
    pub fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
    pub fn glUniform1i(location: GLint, v0: GLint);
    pub fn glUniform2i(location: GLint, v0: GLint, v1: GLint);
    pub fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint);
    pub fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
    pub fn glUniform1fv(location: GLint, count: GLsizei, value: *GLfloat /*const*/);
    pub fn glUniform2fv(location: GLint, count: GLsizei, value: *GLfloat /*const*/);
    pub fn glUniform3fv(location: GLint, count: GLsizei, value: *GLfloat /*const*/);
    pub fn glUniform4fv(location: GLint, count: GLsizei, value: *GLfloat /*const*/);
    pub fn glUniform1iv(location: GLint, count: GLsizei, value: *GLint /*const*/);
    pub fn glUniform2iv(location: GLint, count: GLsizei, value: *GLint /*const*/);
    pub fn glUniform3iv(location: GLint, count: GLsizei, value: *GLint /*const*/);
    pub fn glUniform4iv(location: GLint, count: GLsizei, value: *GLint /*const*/);
    pub fn glUniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glIsShader(shader: GLuint) -> GLboolean ;
    pub fn glIsProgram(program: GLuint) -> GLboolean ;
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *GLint);
    pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *GLint);
    pub fn glGetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *GLsizei, shaders: *GLuint);
    pub fn glGetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar);
    pub fn glGetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *GLsizei, infoLog: *GLchar);
    pub fn glGetUniformLocation(program: GLuint, name: *GLchar /*const*/) -> GLint ;
    pub fn glGetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLint, _type: *GLenum, name: *GLchar);
    pub fn glGetUniformfv(program: GLuint, location: GLint, params: *GLfloat);
    pub fn glGetUniformiv(program: GLuint, location: GLint, params: *GLint);
    pub fn glGetShaderSource(shader: GLuint, bufSize: GLsizei, length: *GLsizei, source: *GLchar);
    pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *GLchar /*const*/);
    pub fn glGetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *GLsizei, size: *GLint, _type: *GLenum, name: *GLchar);
    pub fn glGetAttribLocation(program: GLuint, name: *GLchar /*const*/) -> GLint ;
    pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, _ref: GLint, mask: GLuint);
    pub fn glStencilOpSeparate(face: GLenum, _fail: GLenum, zfail: GLenum, zpass: GLenum);
    pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);
    pub fn glUniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);
    pub fn glUniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat /*const*/);

}
