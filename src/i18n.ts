import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import usCommon from "./locales/en-US/common.json";
import usToolbar from "./locales/en-US/toolbar.json";
import usMenuBar from "./locales/en-US/menubar.json";
import usLanguages from "./locales/en-US/languages.json";
import usSettings from "./locales/en-US/settings.json";
import usForm from "./locales/en-US/form.json"

import huCommon from "./locales/hu-HU/common.json";
import huToolbar from "./locales/hu-HU/toolbar.json";
import huMenuBar from "./locales/hu-HU/menubar.json";
import huLanguages from "./locales/hu-HU/languages.json";
import huSettings from "./locales/hu-HU/settings.json";
import huForm from "./locales/hu-HU/form.json"

import esCommon from "./locales/es-ES/common.json";
import esToolbar from "./locales/es-ES/toolbar.json";
import esMenuBar from "./locales/es-ES/menubar.json";
import esLanguages from "./locales/es-ES/languages.json";
import esSettings from "./locales/es-ES/settings.json";
import esForm from "./locales/es-ES/form.json"

import jpCommon from "./locales/ja-JP/common.json";
import jpToolbar from "./locales/ja-JP/toolbar.json";
import jpMenuBar from "./locales/ja-JP/menubar.json";
import jpLanguages from "./locales/ja-JP/languages.json";
import jpSettings from "./locales/ja-JP/settings.json"; 
import jpForm from "./locales/ja-JP/form.json"

export enum Locale {
  US = "en-US",
  HU = "hu-HU",
  ES = "es-ES",
  JP = "ja-JP",
}

const resources = {
  [Locale.US]: {
    common: usCommon,
    toolbar: usToolbar,
    menubar: usMenuBar,
    languages: usLanguages,
    settings: usSettings,
    form: usForm
  },
  [Locale.HU]: {
    common: huCommon,
    toolbar: huToolbar,
    menubar: huMenuBar,
    languages: huLanguages,
    settings: huSettings,
    form: huForm,
  },
  [Locale.ES] : {
    common: esCommon,
    toolbar: esToolbar,
    menubar: esMenuBar,
    languages: esLanguages,
    settings: esSettings,
    form: esForm
  },
  [Locale.JP] : {
    common: jpCommon,
    toolbar: jpToolbar,
    menubar: jpMenuBar,
    languages: jpLanguages,
    settings: jpSettings,
    form: jpForm
  }
};

i18n.use(initReactI18next).init({
  resources,
  lng: Locale.US,
  fallbackLng: Locale.US,
});
