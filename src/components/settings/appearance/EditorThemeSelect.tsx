import { Select } from "../../ui/Select"
import { useTheme } from "../../../providers/ThemeProvider"
import { /*openFileDialog,*/ useFileOpenDialog } from "../../../hooks/system/useFileOpenDialog"
import { Button } from "../../ui/Button"
import { useTranslation } from "react-i18next"

/*
    import 'ace-builds/src-noconflict/theme-nord_dark';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/theme-ambiance';
import 'ace-builds/src-noconflict/theme-tomorrow_night';
import 'ace-builds/src-noconflict/theme-tomorrow_night_blue';
import 'ace-builds/src-noconflict/theme-tomorrow_night_bright';
import 'ace-builds/src-noconflict/theme-tomorrow_night_eighties';
import 'ace-builds/src-noconflict/theme-xcode';
import 'ace-builds/src-noconflict/theme-textmate';
import 'ace-builds/src-noconflict/theme-ambiance';
import 'ace-builds/src-noconflict/theme-chaos';
import 'ace-builds/src-noconflict/theme-chrome';
import 'ace-builds/src-noconflict/theme-clouds';
import 'ace-builds/src-noconflict/theme-cobalt';
import 'ace-builds/src-noconflict/theme-crimson_editor';
import 'ace-builds/src-noconflict/theme-dawn';
import 'ace-builds/src-noconflict/theme-dracula';
import 'ace-builds/src-noconflict/theme-dreamweaver';
import 'ace-builds/src-noconflict/theme-eclipse';
import 'ace-builds/src-noconflict/theme-github';
import 'ace-builds/src-noconflict/theme-gob';
import 'ace-builds/src-noconflict/theme-gruvbox';
import 'ace-builds/src-noconflict/theme-idle_fingers';
import 'ace-builds/src-noconflict/theme-kr_theme';
import 'ace-builds/src-noconflict/theme-kuroir';
import 'ace-builds/src-noconflict/theme-merbivore';
import 'ace-builds/src-noconflict/theme-merbivore_soft';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/theme-monokai_soft';
import 'ace-builds/src-noconflict/theme-pastel_on_dark';
import 'ace-builds/src-noconflict/theme-solarized_dark';
import 'ace-builds/src-noconflict/theme-solarized_light';
import 'ace-builds/src-noconflict/theme-sqlserver';
*/

export const EditorThemeSelect = () => {
    const {editorTheme,setEditorTheme,setEditorImage} = useTheme();
    const {t} = useTranslation("settings");
   // const {openDialog, selectedFile} = useFileOpenDialog();

   /* const onImageChange = (e:Event) => {
        console.log("File path/name",e.target.files[0].name)
    }*/

    const openFileDialog = async () => {
        const selectedFile = await useFileOpenDialog();
        console.log("Selected file: ", selectedFile);
        setEditorImage(selectedFile);
    }

    return (
        <div>
            <label>{t("appearance.theme.editor.image")}</label>
            <Button variant="outline" onClick={openFileDialog}>{t("appearance.theme.editor.selectFile")}</Button>
            <label>{t("appearance.theme.editor.baseTheme")}</label>
            <Select options={[
                "nord_dark",
                "monokai",
                "ambiance",
                "tomorrow_night",
                "tomorrow_night_blue",
                "tomorrow_night_bright",
                "tomorrow_night_eighties",
                "xcode",
                "textmate",
                "chaos",
                "chrome",
                "clouds",
                "cobalt",
                "crimson_editor",
                "dawn",
                "dracula",
                "dreamweaver",
                "eclipse",
                "github",
                "gob",
                "gruvbox",
                "idle_fingers",
                "kr_theme",
                "kuroir",
                "merbivore",
                "merbivore_soft",
                "monokai_soft",
                "pastel_on_dark",
                "solarized_dark",
                "solarized_light",
                "sqlserver",
            ]} onValueChange={(e) => setEditorTheme(e)} value={editorTheme}/>
        </div>
    )
}