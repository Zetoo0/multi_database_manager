import "i18next";
import common from "./locales/en-US/common.json";
import menubar from "./locales/en-US/menubar.json";
import toolbar from "./locales/en-US/toolbar.json";
import languages from "./locales/en-US/languages.json";
import settings from "./locales/en-US/settings.json";
import form from "./locales/en-US/form.json"
import object from "./locales/en-US/object.json"
declare module "i18next" {
  interface CustomTypeOptions {
    defaultNS: "en-US";
    resources: {
      common: typeof common;
      menubar: typeof menubar;
      toolbar: typeof toolbar;
      languages: typeof languages;
      settings: typeof settings;
      form: typeof form;
      object: typeof object
    };
  }
}
