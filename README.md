<p align="center">
  <img src="docs/images/icon.png" width="112" height="112" alt="FM27 Trainer icon">
</p>

<h1 align="center">FM27 Trainer (TR Fork)</h1>

<p align="center">
  <strong>Football Manager 27 için hilesiz, akıllı takım antrenmanı otomasyonu ve BepInEx 17/17 Taktiksel Verimlilik okuyucu.<br>
  Legitimate, hands-free training automation and exact BepInEx Tactical Effectiveness reader for Football Manager 27.</strong>
</p>

<p align="center">
  <img alt="Windows" src="https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows">
  <img alt="FM27" src="https://img.shields.io/badge/Football%20Manager-27-c8102e">
  <img alt="Rust" src="https://img.shields.io/badge/built%20with-Rust-dea584?logo=rust">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-7c6cff">
</p>

<p align="center">
  <a href="#türkçe">🇹🇷 Türkçe</a> ·
  <a href="#english">🇬🇧 English</a> ·
  <a href="../../releases/latest">Releases</a> ·
  <a href="docs/TECHNICAL.md">Technical notes</a>
</p>

<p align="center">
  <img src="docs/images/training.png" alt="FM27 Trainer" width="92%">
</p>

---

<a name="türkçe"></a>
## 🇹🇷 Türkçe

Bu proje, **[SemHiel2003/fm27-trainer](https://github.com/SemHiel2003/fm27-trainer)** (MIT Lisansı) temel alınarak **hilesiz antrenman otomasyonu**, **OCR'sız kesin Taktiksel Verimlilik okuma (BepInEx köprüsü)** ve **tam Türkçe arayüz** odağıyla geliştirilmiş bir topluluk fork'udur.

### Öne Çıkan Özellikler
- **Opsiyonel BepInEx Köprüsü (`FM27Trainer.Bridge.dll`):** OCR'a gerek kalmadan FM27'nin kendi `TeamTactic.TacticalEffectiveness` verisini doğrudan oyunun içinden okur — **4 ana bileşen ve 17 kilit faktörün tamamı 0–100 tam sayı hassasiyetiyle**.
- **Türkçe Arayüz & OCR Düzeltmesi:** Sistem dilinden otomatik Türkçe algılama (`Türkçe / English / Nederlands`) ve Türkçe OCR'daki kısa *"Pas Stili"* etiketi kesilme hatasının giderilmesi (`17/17` hatasız okuma).
- **%100 Saf Hilesiz Antrenman Sürümü:** Tüm editör/hile sekmeleri (`Kulüp`, `Kadro`, `Transferler`) ve yazma komutları (`bütçe, iyileştir, ceza kaldır, moral, taktik %100, al/sat`) uygulamadan tamamen kaldırılmıştır — teknik olarak yalnızca antrenman takvimine yazabilir.
- **İlişkiler Odaklı Oyuncu Benimsemesi (`Buy-In`):** `Takım Kaynaşması` yalnızca antrenmanla artabilen *Takım Arkadaşları ve Personel İlişkileri* skoruna göre tetiklenir; ilişkiler çok düşükse (`< %45`) maç ertesi güne otomatik `Sosyal Yardımlaşma (Community Outreach)` de eklenir.
- **Taktik Tarzı (`TacticMode`):** *Otomatik / Gegenpress / Tiki-Taka / Kanat & Hızlı Hücum / Savunma & Kontra* — taktik aşamasında dengeli öncelik (`1.25×` / `0.90×`), Gelişim modunda seçili oyun tarzına özel 7 seanslık gelişim havuzu.
- **Özel Kamp Modları (`CampMode`):** *Standart / Kondisyon Kampı (1 hafta) / Oyuncu Gelişim Kampı / Derbi & Final Kampı (1 hafta)* — düşük kadro kondisyonu veya `2+` maçlı haftalarda güvenlik korumaları her zaman öncelikli çalışır.
- **Akıllı Otomatik Korumalar:**
  - Sağlıklı kadro ortalama kondisyonu eşiğin (varsayılan `%78`) altına düştüğünde veya haftada `2+` maç olduğunda otomatik **Daha fazla dinlenme (`Light`)**.
  - 8 taktik alanın tamamı eşiğin (varsayılan `%85`) üstüne çıktığında otomatik **Gelişim modu**; herhangi biri düştüğünde tekrar taktiğe dönüş.
  - Fikstürdeki maçsız haftaları otomatik **Milli Ara / Kış Arası** olarak tanıma.
  - **Antrenmanı otomatik planda tut:** FM yardımcı antrenörünün haftalık takvimi bozmasına karşı 8 saniyede bir kontrol edip geri yükler.
- **Haftalık 5 Metrikli Etki Özeti:** Her hafta kartının altında FM27'nin gerçek 59 seans verisine göre *Kondisyon, Yorgunluk, Mutluluk, Sakatlık Riski ve Keskinlik* özeti.
- **Otomatik Takvim Doğrulaması:** Yeni FM27 yamalarında takvim yapısı 7 aşamalı kontrolden geçtiğinde (ardışık haftalar, geçerli seans kodları, eski bayt kontrolü ve geri alma) yalnızca antrenman takvimi yazımına güvenle izin verir; diğer tüm editör yazmaları kilitli kalır.

### Kurulum (Windows)
1. **[Releases](../../releases/latest)** sayfasından **`FM27 Trainer.exe`** dosyasını indirip doğrudan çalıştırın (ek kurulum gerekmez).
2. *(Opsiyonel — OCR'sız 17/17 kesin okuma için)* Oyununuzda BepInEx 6 (Unity IL2CPP) kuruluysa **`FM27Trainer.Bridge.dll`** dosyasını `<FM27>\BepInEx\plugins\FM27Trainer.Bridge\` klasörüne kopyalayıp FM27'yi yeniden başlatın.
3. FM27'yi açıp kaydınızı yükleyin; uygulama otomatik bağlanır.

---

<a name="english"></a>
## 🇬🇧 English

A community fork of **[SemHiel2003/fm27-trainer](https://github.com/SemHiel2003/fm27-trainer)** (MIT License) focused on legitimate, hands-free training automation, exact OCR-free Tactical Effectiveness reading via an optional BepInEx 6 bridge, and full Turkish support.

### Key Features
- **Optional BepInEx Bridge (`FM27Trainer.Bridge.dll`):** Reads FM27's `TeamTactic.TacticalEffectiveness` directly from the game — all **4 components and 17 key factors with exact 0–100 integer precision**, without needing OCR.
- **Turkish UI & OCR Fix:** Full Turkish interface (`Türkçe / English / Nederlands`) with automatic OS locale detection and fixed pie-gauge cutoff for short Turkish labels (*"Pas Stili"*).
- **100% Training-Only Build (No Cheats):** All editor/cheat tabs (`Club`, `Squad`, `Transfers`) and write commands (`finances, heal, lift bans, morale, max familiarity, buy/sell`) are completely removed from the release binary — it can technically only write to the training calendar.
- **Relationships-Focused Player Buy-In:** `Team Bonding` targets only the trainable *Teammates and Staff Relationships* score (never wasting sharpness on untrainable *Managerial Support*), and adds `Community Outreach` on post-match days when relationships drop below `45%`.
- **Tactic Style (`TacticMode`):** *Auto / Gegenpress / Possession / Wing & Fast Attack / Low Block & Counter* with balanced priority weights (`1.25×` / `0.90×`) and a style-specific development session pool.
- **Special Camp Modes (`CampMode`):** 1-week scoped *Fitness Camp* and *Derby & Final Prep* plus *Player Development*, always yielding to fatigue/congestion safety guards.
- **Automated Training Guards:**
  - Auto *More rest (`Light`)* when healthy squad condition drops below threshold (default `78%`) or in weeks with `2+` matches.
  - Auto switch to player development once all 8 tactical areas exceed threshold (default `85%`), switching back automatically if any area drops.
  - Automatic *International Break / Winter Break* detection on 0-match weeks.
  - Automatic calendar protection (*Keep training on plan automatically*) against FM's assistant overwriting upcoming weeks.
- **Weekly 5-Metric Impact Summary:** Displays *Condition, Fatigue, Happiness, Injury Risk, Sharpness* totals on every week card using FM27's official 59-session data.
- **Auto-Verified Training Calendar Writes:** Safely allows training-only calendar writes on newer FM27 builds once all structural safety checks pass (ascending Monday weeks, valid session codes, byte-level check & undo), while keeping all other editor writes read-only.

### Install (Windows)
1. Download **`FM27 Trainer.exe`** from **[Releases](../../releases/latest)** and run it directly.
2. *(Optional — for exact OCR-free 17/17 reading)* If you have BepInEx 6 (Unity IL2CPP) installed, drop **`FM27Trainer.Bridge.dll`** into `<FM27>\BepInEx\plugins\FM27Trainer.Bridge\` and restart FM27.

### Build from Source
```bash
# Desktop app (Rust stable)
cargo build --release -p fm27-trainer

# Optional BepInEx 6 IL2CPP bridge (.NET 8 SDK)
dotnet build bridge/FM27Trainer.Bridge.csproj -c Release -p:FM27Dir="PATH_TO_FM27"
```

---

## Credits & License

- **Original project:** [SemHiel2003/fm27-trainer](https://github.com/SemHiel2003/fm27-trainer) by **Sem Hiel** (MIT License).
- **Research & layouts:** FMSuperScout, FMLibreEditor, fm-lens, fmsave. Font: Inter (SIL OFL). Icons: Phosphor (MIT). See [THIRD_PARTY.md](THIRD_PARTY.md).
- **License:** [MIT](LICENSE)
