import { Translation, useTranslation } from "react-i18next";
import { useTheme } from "../../../providers/ThemeProvider"
import { Input } from "../../ui/Input"
import { Button } from "../../ui/Button";


/*const DEFAULT_THEME = {
    background: "39 52% 90%",
    foreground: "20 15% 20%",
    primary: "34 53% 45%",
    "primary-foreground": "39 52% 95%",
    secondary: "39 30% 80%",
    "secondary-foreground": "20 15% 10%",
    accent: "34 50% 50%",
    "accent-foreground": "39 52% 90%",
    border: "39 30% 60%",
  }*/


  const hexToHSL = (hex: string) => {
    let r = 0, g = 0, b = 0, s=0;
    if (hex.length === 7) { // #RRGGBB
      r = parseInt(hex.substring(1, 3), 16);
      g = parseInt(hex.substring(3, 5), 16);
      b = parseInt(hex.substring(5, 7), 16);
    }
  
    r /= 255, g /= 255, b /= 255;
    let max = Math.max(r, g, b), min = Math.min(r, g, b);
    let h = 0, l = (max + min) / 2;
    s = (max + min) / 2;
    if (max === min) h = 0; 
    else {
      let d = max - min;
      s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
      switch (max) {
        case r: h = (g - b) / d + (g < b ? 6 : 0); break;
        case g: h = (b - r) / d + 2; break;
        case b: h = (r - g) / d + 4; break;
      }
      h *= 60;
    }
    return `${Math.round(h)} ${Math.round(s * 100)}% ${Math.round(l * 100)}%`;
  };
  
  const hslToHex = (hsl: string) => {
    const [h, s, l] = hsl.split(" ").map((v, i) => (i === 0 ? parseFloat(v) : parseFloat(v) / 100));
    const a = s * Math.min(l, 1 - l);
    const f = (n: number) => {
      const k = (n + h / 30) % 12;
      const color = l - a * Math.max(-1, Math.min(k - 3, 9 - k, 1));
      return Math.round(255 * color)
        .toString(16)
        .padStart(2, "0");
    };
    return `#${f(0)}${f(8)}${f(4)}`;
  };
  

export const ApplicationColorChanger = () => {
    
    const {updateThemeInfo,themeInfo,saveTheme} = useTheme();
    const {t} = useTranslation("settings")
    console.log("Theme info: ",themeInfo);    
    return (
        <div className="p-4 space-y-4">
        <h2 className="text-lg font-semibold">🎨{t("appearance.theme.color.customizeTheme")}</h2>
        {Object.keys(themeInfo).map((key) => (
          <div key={key} className="flex items-center gap-2">
            <label className="w-40 capitalize">{t(`appearance.theme.color.${key.replace("-", " ")}` as keyof typeof Translation)}</label>
            <Input type="color" value={hslToHex(themeInfo[key])} onChange={(e) => updateThemeInfo(key,hexToHSL(e.target.value))}/>
          </div>
        ))}
        <Button onClick={saveTheme}>Save</Button>
      </div>
    )
}