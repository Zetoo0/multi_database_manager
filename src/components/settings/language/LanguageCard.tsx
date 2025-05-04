import * as RadioGroup from "@radix-ui/react-radio-group";
import { memo, useRef } from "react";
import { Locale } from "../../../i18n";
import { useTranslation } from "react-i18next";
import { useLayout } from "../../../providers/LayoutProvider";
  
interface LanguageCardProps {
  lang: Locale;
}

export const LanguageCard = ({ lang }: LanguageCardProps) => {
  const { t } = useTranslation("languages");
  const radioRef = useRef<HTMLButtonElement | null>(null);

  const { readingDirection } = useLayout();

  const handleSelect = () => {
    radioRef.current?.click();
  };

  return (
    <RadioGroup.Item
      ref={radioRef}
      value={lang}
      id={`lang-${lang}`}
      dir={readingDirection}
      asChild
    >
      <li
        className="group flex cursor-pointer items-center gap-2 rounded border p-2 px-4 transition-colors hover:bg-accent data-[state=checked]:bg-muted"
        onClick={handleSelect}
      >
        <div className="flex size-5 items-center justify-center rounded-full border-2 text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 group-data-[state=checked]:border-primary">
          <RadioGroup.Indicator className="flex items-center justify-center">
            <div className="size-3 rounded-full bg-primary"></div>
          </RadioGroup.Indicator>
        </div>

        <label
          htmlFor={`lang-${lang}`}
          className="mr-8 cursor-pointer rtl:ml-8 rtl:mr-0"
        >
          <LocalalizedLocale lang={lang} />
        </label>

        <div className="ml-auto text-sm text-muted-foreground rtl:ml-0 rtl:mr-auto">
          {t(lang)}
        </div>

        <img
          src={`/lang/flags/${lang}.svg`}
          className="ml-2 h-6 w-auto rounded-sm"
        />
      </li>
    </RadioGroup.Item>
  );
};

const LocalalizedLocale = memo(({ lang }: { lang: Locale }) => {
  const { t } = useTranslation("languages", { lng: lang });

  return <>{t(lang)}</>;
});
LocalalizedLocale.displayName = "LocalalizedLocale";
