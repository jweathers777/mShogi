//------------------------------------------------------------------------------
// Header generated by wxDesigner from file: mShogi.wdr
// Do not modify this file, all changes will be lost!
//------------------------------------------------------------------------------

#ifndef __WDR_mShogi_H__
#define __WDR_mShogi_H__

#if defined(__GNUG__) && !defined(NO_GCC_PRAGMA)
    #pragma interface "mShogi_wdr.h"
#endif

// Include wxWidgets' headers

#ifndef WX_PRECOMP
    #include <wx/wx.h>
#endif

#include <wx/image.h>
#include <wx/statline.h>
#include <wx/spinbutt.h>
#include <wx/spinctrl.h>
#include <wx/splitter.h>
#include <wx/listctrl.h>
#include <wx/treectrl.h>
#include <wx/notebook.h>
#include <wx/grid.h>
#include <wx/toolbar.h>

// Declare window functions

const int ID_BTN_FLIP = 10000;
const int ID_BMP_FLIPBOARD = 10001;
wxSizer *InitOrientationPage( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_SPN_TOP_LEFT = 10002;
const int ID_TXT_TOP_LEFT = 10003;
const int ID_SPN_TOP_RIGHT = 10004;
const int ID_TXT_TOP_RIGHT = 10005;
const int ID_CHOICE_TIMESTYLE = 10006;
const int ID_RDB_SAME = 10007;
const int ID_RDB_SEPARATE = 10008;
const int ID_SUDDEN_DEATH = 10009;
const int ID_SPN_MIDDLE = 10010;
const int ID_TXT_MIDDLE = 10011;
const int ID_SPN_BOTTOM_LEFT = 10012;
const int ID_TXT_BOTTOM_LEFT = 10013;
const int ID_SPN_BOTTOM_RIGHT = 10014;
const int ID_TXT_BOTTOM_RIGHT = 10015;
wxSizer *InitTimeControlsPage( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_LST_GAME = 10016;
const int ID_LST_STYLE = 10017;
wxSizer *InitGamePage( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_TXT_WHITE = 10018;
const int ID_RDB_WPLAYER = 10019;
const int ID_BTN_WSELECT = 10020;
const int ID_BTN_WEDIT = 10021;
const int ID_NTB_SETUP = 10022;
const int ID_TXT_BOTTOM = 10023;
const int ID_RDB_BPLAYER = 10024;
const int ID_BTN_BSELECT = 10025;
const int ID_BTN_BEDIT = 10026;
const int ID_LINE = 10027;
wxSizer *InitSetupNotebook( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_BMP_WHITE = 10028;
const int ID_TXT_WPLAYER = 10029;
const int ID_TXT_WTIMECTRL = 10030;
const int ID_TXT_WCLOCK = 10031;
const int ID_BTN_FIRST = 10032;
const int ID_BTN_BACK = 10033;
const int ID_BTN_REWIND = 10034;
const int ID_BTN_STOP = 10035;
const int ID_BTN_PLAY = 10036;
const int ID_BTN_FORWARD = 10037;
const int ID_BTN_LAST = 10038;
const int ID_LST_MOVES = 10039;
const int ID_BMP_BLACK = 10040;
const int ID_TXT_BPLAYER = 10041;
const int ID_TXT_BTIMECTRL = 10042;
const int ID_TXT_BCLOCK = 10043;
wxSizer *InitGameStatusWnd( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_LST_NODES = 10044;
const int ID_LST_BESTNODE = 10045;
wxSizer *InitThinkingWnd( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_TXT_ANNOTATION = 10046;
const int ID_BTN_EDIT = 10047;
wxSizer *InitAnnotationWnd( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_LST_CAPTURED = 10048;
wxSizer *InitCapturesWnd( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_STAT_PERSONALITY = 10049;
const int ID_TXT_PERSONALITY = 10050;
const int ID_BTN_PSELECT = 10051;
const int ID_BTN_CLOSE = 10052;
const int ID_STAT_OPENING = 10053;
const int ID_TXT_OPENING = 10054;
const int ID_BTN_OSELECT = 10055;
const int ID_BTN_SAVE = 10056;
const int ID_STAT_BASE_RATING = 10057;
const int ID_TXT_BASE_RATING = 10058;
const int ID_BTN_DELETE = 10059;
const int ID_NTB_PERSONALITY = 10060;
wxSizer *InitSetupPersonality( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_STAT_TITLE = 10061;
const int ID_TXT_TITLE = 10062;
const int ID_STAT_BLACK = 10063;
const int ID_TXT_BLACK = 10064;
const int ID_STAT_BTITLE = 10065;
const int ID_TXT_BTITLE = 10066;
const int ID_STAT_BRATING = 10067;
const int ID_TXT_BRATING = 10068;
const int ID_STAT_BCOUNTRY = 10069;
const int ID_TXT_BCOUNTRY = 10070;
const int ID_STAT_WHITE = 10071;
const int ID_STAT_WTITLE = 10072;
const int ID_TXT_WTITLE = 10073;
const int ID_TXT_WRATING = 10074;
const int ID_STAT_WCOUNTRY = 10075;
const int ID_TXT_WCOUNTRY = 10076;
const int ID_STAT_EVENT = 10077;
const int ID_TXT_EVENT = 10078;
const int ID_STAT_ROUND = 10079;
const int ID_TXT_ROUND = 10080;
const int ID_STAT_SITE = 10081;
const int ID_TXT_SITE = 10082;
const int ID_STAT_DATE = 10083;
const int ID_TXT_DATE = 10084;
const int ID_STAT_ANNOTATOR = 10085;
const int ID_TXT_ANNOTATOR = 10086;
const int ID_STAT_ARATING = 10087;
const int ID_TXT_ARATING = 10088;
const int ID_STAT_REMARKS = 10089;
const int ID_TXT_REMARKS = 10090;
const int ID_RDB_RESULTS = 10091;
wxSizer *InitGameInfo( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_STAT_ATTDEF = 10092;
const int ID_SLD_ATTDEF = 10093;
const int ID_TXT_ATTDEF = 10094;
const int ID_STAT_STRENGTH = 10095;
const int ID_SLD_STRENGTH = 10096;
const int ID_TXT_STRENGTH = 10097;
const int ID_STAT_MATDEF = 10098;
const int ID_SLD_MATDEF = 10099;
const int ID_STAT_RANDOMNESS = 10100;
const int ID_SLD_RANDOMNESS = 10101;
const int ID_TXT_RANDOMNESS = 10102;
const int ID_STAT_MAXDEPTH = 10103;
const int ID_SLD_MAXDEPTH = 10104;
const int ID_TXT_MAXDEPTH = 10105;
const int ID_STAT_SELECTIVE = 10106;
const int ID_SLD_SELECTIVE = 10107;
const int ID_TXT_SELECTIVE = 10108;
const int ID_STAT_CONTEMPT = 10109;
const int ID_SLD_CONTEMPT = 10110;
const int ID_TXT_CONTEMPT = 10111;
const int ID_STAT_TRANS_TABLE = 10112;
const int ID_SLD_TRANS_TABLE = 10113;
const int ID_STAT_TRANS_VALUE = 10114;
const int ID_CHK_PONDERING = 10115;
const int ID_CHK_ENDGAME_DATABASE = 10116;
wxSizer *InitStylePage( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_STAT_OWNCENTER = 10117;
const int ID_SLD_OWNCENTER = 10118;
const int ID_TXT_OWNCENTER = 10119;
const int ID_STAT_OPPOSECENTER = 10120;
const int ID_SLD_OPPOSECENTER = 10121;
const int ID_TXT_OPPOSECENTER = 10122;
const int ID_STAT_OWNMOBILITY = 10123;
const int ID_SLD_OWNMOBILITY = 10124;
const int ID_TXT_OWNMOBILITY = 10125;
const int ID_STAT_OPPOSEMOBILITY = 10126;
const int ID_SLD_OPPOSEMOBILITY = 10127;
const int ID_TXT_OPPOSEMOBILITY = 10128;
const int ID_STAT_OWNKING = 10129;
const int ID_SLD_OWNKING = 10130;
const int ID_TXT_OWNKING = 10131;
const int ID_STAT_OPPOSEKING = 10132;
const int ID_SLD_OPPOSEKING = 10133;
const int ID_TXT_OPPOSEKING = 10134;
const int ID_STAT_OWNPAWNS = 10135;
const int ID_SLD_OWNPAWNS = 10136;
const int ID_TXT_OWNPAWNS = 10137;
const int ID_STAT_OPPOSEPAWNS = 10138;
const int ID_SLD_OPPOSEPAWNS = 10139;
const int ID_TXT_OPPOSEPAWNS = 10140;
const int ID_STAT_OWNWEAKNESS = 10141;
const int ID_SLD_OWNWEAKNESS = 10142;
const int ID_TXT_OWNWEAKNESS = 10143;
const int ID_STAT_OPPOSEWEAKNESS = 10144;
const int ID_SLD_OPPOSEWEAKNESS = 10145;
const int ID_TXT_OPPOSEWEAKNESS = 10146;
wxSizer *InitPositionalPage( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

const int ID_BMP_NE_LOGO = 10147;
const int ID_TXT_DESCRIP = 10148;
const int ID_TXT_VERSION = 10149;
const int ID_TXT_COPYRIGHT = 10150;
wxSizer *InitAboutDlg( wxWindow *parent, bool call_fit = TRUE, bool set_sizer = TRUE );

// Declare menubar functions

const int ID_FILE = 10151;
const int ID_LOAD = 10152;
const int ID_SAVE = 10153;
const int ID_MENU = 10154;
const int ID_PRINT = 10155;
const int ID_PRINT_MOVELIST = 10156;
const int ID_PRINT_POSITION = 10157;
const int ID_SETUP_PRINTER = 10158;
const int ID_EXIT = 10159;
const int ID_EDIT = 10160;
const int ID_COPY = 10161;
const int ID_COPY_PGN = 10162;
const int ID_COPY_MOVELIST = 10163;
const int ID_MOVELIST_TABS = 10164;
const int ID_COPY_FORSYTHE = 10165;
const int ID_COPY_ASCII = 10166;
const int ID_COPY_BITMAP = 10167;
const int ID_PASTE = 10168;
const int ID_PASTE_PGN = 10169;
const int ID_PASTE_FORSYTHE = 10170;
const int ID_GAME = 10171;
const int ID_NEW = 10172;
const int ID_GAME_INFORMATION = 10173;
const int ID_SETUP_GAME = 10174;
const int ID_SETUP_POSITION = 10175;
const int ID_SETUP_PERSONALITY = 10176;
const int ID_ACTIONS = 10177;
const int ID_TAKEBACK = 10178;
const int ID_REPLAY = 10179;
const int ID_FORCE = 10180;
const int ID_WAKEUP = 10181;
const int ID_PAUSE = 10182;
const int ID_OFFER_DRAW = 10183;
const int ID_RESIGN = 10184;
const int ID_SWITCH_SIDES = 10185;
const int ID_MENTOR = 10186;
const int ID_QUICK_HINT = 10187;
const int ID_ADVICE = 10188;
const int ID_SOLVE_MATE = 10189;
const int ID_PREFERENCES = 10190;
const int ID_SHOGI_SET = 10191;
const int ID_FLIP_BOARD = 10192;
const int ID_SOUND = 10193;
const int ID_BOARD_SETTINGS = 10194;
const int ID_NOTATION = 10195;
const int ID_COORDINATE = 10196;
const int ID_ALGEBRAIC = 10197;
const int ID_LONG_ALGEBRAIC = 10198;
const int ID_INTERNATIONAL = 10199;
const int ID_LAYOUT = 10200;
const int ID_LAYOUT_LOAD = 10201;
const int ID_LAYOUT_SAVE = 10202;
const int ID_LAYOUT_RESTORE = 10203;
const int ID_WINDOWS = 10204;
const int ID_GAME_STATUS = 10205;
const int ID_ANNOTATION = 10206;
const int ID_CAPTURED_PIECES = 10207;
const int ID_THINKING_LINES = 10208;
const int ID_HELP = 10209;
const int ID_CONTENTS = 10210;
const int ID_GLOSSARY = 10211;
const int ID_ABOUT = 10212;
wxMenuBar *MyMenuBarFunc();

// Declare toolbar functions

void MyToolBarFunc( wxToolBar *parent );

// Declare bitmap functions

wxBitmap InitBitmaps( size_t index );

#endif

// End of generated file