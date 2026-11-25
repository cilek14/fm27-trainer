<p align="center">
  <img src="docs/images/icon.png" width="112" height="112" alt="FM27 Training Assistant icon">
</p>

<h1 align="center">FM27 Training Assistant</h1>

<p align="center">
  <strong>Football Manager 27 için hilesiz akıllı antrenman asistanı.<br>
  A no-cheat smart training assistant for Football Manager 27.</strong>
</p>

<p align="center">
  <img alt="Windows" src="https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows">
  <img alt="FM27" src="https://img.shields.io/badge/Football%20Manager-27-c8102e">
  <img alt="BepInEx" src="https://img.shields.io/badge/BepInEx-6%20IL2CPP-5c2d91">
  <img alt="No cheats" src="https://img.shields.io/badge/hile-yok%20%2F%20no%20cheats-3dd68c">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-7c6cff">
</p>

<p align="center">
  <a href="#türkçe">🇹🇷 Türkçe</a> ·
  <a href="#english">🇬🇧 English</a> ·
  <a href="../../releases/latest">İndir / Download</a>
</p>

<p align="center">
  <img src="docs/images/training.png" alt="FM27 Training Assistant" width="92%">
</p>

---

<a name="türkçe"></a>
## 🇹🇷 Türkçe

### Bu program ne işe yarar?

FM27'de antrenmanı her hafta elle ayarlamak zahmetlidir. **FM27 Training Assistant** bu işi senin yerine yapar:

- Takımının **hangi taktik konularda zayıf olduğuna** bakar ve haftalık antrenmanı ona göre hazırlar.
- **Zaman içindeki taktiksel gelişimini** (`▲/▼` haftalık değişim ve trend grafiğiyle) takip eder.
- Yorgun oyuncuları ve yoğun fikstürü fark edip **gerektiğinde dinlenmeye ağırlık verir**.
- **Taktiğe en az uyum sağlayan oyuncuları** listeler.
- Her oyuncu için **sakatlık riskini düşüren antrenman yoğunluğunu** ve **rol zorlamadan Ek Odak (Additional Focus)** önerir.

### Hile var mı?

**Hayır.** Program yalnızca FM'de senin de elle yapabileceğin şeyleri yapar:

| ✅ Yapar | ❌ Yapmaz |
|---|---|
| Antrenman takvimini doldurur | Oyuncu özelliklerini yükseltmez |
| Oyuncuların antrenman yoğunluğunu ve Ek Odak'ını seçer | Moral, kondisyon veya form değiştirmez |
| Kadro ve özellik bilgilerini **salt-okunur** okur | Para, transfer, sakatlık veya ceza ile oynamaz |

### Ne yapabilirsin?

**📊 Genel Bakış — Taktiksel Verimlilik & Gelişim Trendi**
- 4 ana bileşen ve 17 faktör tamamen Türkçe gösterilir.
- Kulüp bazlı geçmiş tutulur: bir önceki ölçüme göre değişimler (`▲+5`, `▼-2`) ve gelişim grafiği görünür.

**📅 Antrenman sekmesi — takım antrenmanı**
- 62 FM27 seansının tamamı Türkçe adlarıyla gösterilir.
- Oyun tarzını (`Gegenpress`, `Tiki-Taka`, `Kanat`, `Savunma & Kontra`) ve özel kampı (`Kondisyon`, `Oyuncu Gelişimi`, `Derbi/Final`) seçebilirsin.
- Kadro yorgunsa veya haftada `2+` maç varsa otomatik dinlendirir; takım taktiği öğrendiğinde gelişim moduna geçer.
- FM yardımcı antrenörü takvimi değiştirirse planını 8 saniyede bir kontrol edip geri yükler.

**👤 Bireysel sekmesi — radar, darboğaz, yoğunluk ve Ek Odak**
- **Maç Öncesi Risk Radarı:** Haftalık maç sayısı, kritik kondisyonlu oyuncular, korumadaki veteran sayısı ve ortalama kadro hazırlığı.
- **"Taktiği kim aşağı çekiyor?"** listesi: taktiğe uyumu en düşük oyuncuları ve ana darboğazlarını sıralar.
- **Akıllı yoğunluk + 3 Hazır Profil (`Korumacı` / `Dengeli` / `Gelişim Odaklı`):** Yaş, kondisyon, keskinlik, sakatlık ve haftalık maç sayısına göre *Yarı / Normal / Çift* önerir ve tek tıkla FM'ye uygular.
- **Rol Zorlamadan Ek Odak (Additional Focus):** Her oyuncunun yalnızca kendi doğal mevkisindeki en zayıf kilit özellik grubunu bulup önerir ve tek tıkla uygular. Yedek oyuncular asla uygun olmayan taktik rollerine zorlanmaz.

### Kurulum (Windows)

1. **[İndirme sayfasından](../../releases/latest)** `FM27 Trainer.exe` dosyasını indirip çalıştır. Kurulum gerekmez.
2. FM27'yi aç ve kariyerini yükle. Program oyuna kendiliğinden bağlanır.

**İsteğe bağlı (önerilen): BepInEx eklentisi (`v0.7.0`)**

Oyununda BepInEx 6 (Unity IL2CPP) kuruluysa `FM27Trainer.Bridge.dll` dosyasını şu klasöre kopyala:

```
<FM27 klasörü>\BepInEx\plugins\FM27Trainer.Bridge\
```

Sonra FM27'yi yeniden başlat. Eklentiyle:
- Taktik değerleri doğrudan oyundan **kesin sayı olarak** okunur.
- Bireysel sekmesindeki oyuncu darboğaz listesi dolar ve yoğunluk / Ek Odak değişiklikleri tek tıkla oyuna uygulanır.

Eklenti yoksa program ekran görüntüsünden (OCR) okuyarak çalışmaya devam eder.

### Dil

Türkçe, İngilizce ve Hollandaca; sol alttan (`TR | EN | NL`) veya Ayarlar'dan değiştirilebilir.

---

<a name="english"></a>
## 🇬🇧 English

### What does it do?

Setting up training by hand every week in FM27 is tedious. **FM27 Training Assistant** handles it for you:

- Checks **which tactical areas your team is weak in** and builds weekly schedules around them.
- Tracks **Tactical Effectiveness history over time** (`▲/▼` deltas and a progress line chart).
- Notices tired players and busy fixture weeks and **adds rest when needed**.
- Lists **the players who drag tactical effectiveness down**.
- Suggests **injury-safe individual training intensity** and **role-free Additional Focus** per player.

### Is it a cheat?

**No.** It only does things you could also do by hand in FM:

| ✅ It does | ❌ It never does |
|---|---|
| Fill in the training calendar | Boost player attributes |
| Set individual training intensity & Additional Focus | Change morale, condition or sharpness |
| Read squad & attribute data **read-only** | Touch finances, transfers, injuries or bans |

### What can you do with it?

**📊 Overview — Tactical Effectiveness & Progress Trend**
- Tracks changes (`▲+5`, `▼-2`) per club across readings with a 4-component trend chart.

**📅 Training tab — team training**
- Focused weekly schedules for your tactical weaknesses, with playing styles (*Gegenpress, Possession, Wing Attack, Low Block & Counter*) and camp modes.
- Automatically rests fatigued squads or `2+` match weeks, detects international/winter breaks, and restores your plan if FM's assistant changes it.

**👤 Individual tab — radar, bottlenecks, intensity profiles & Additional Focus**
- **Pre-match Risk Radar:** Weekly fixture count, critical fatigue/injury count, guarded veterans, and squad readiness.
- **"Who drags the tactic down?"**: Players sorted from lowest to highest tactical fit with actionable advice.
- **Smart Intensity + 3 Profiles (`Protective` / `Balanced` / `Development`):** Recommends *Half / Normal / Double* and applies legitimately via FM's native UI event.
- **Role-free Additional Focus:** Recommends the best focus based on each player's own natural position and weakest attributes—never forcing backups into unsuitable tactic roles.

### Install

1. Download `FM27 Trainer.exe` from the **[download page](../../releases/latest)** and run it.
2. *(Recommended)* With BepInEx 6 (Unity IL2CPP) installed, copy `FM27Trainer.Bridge.dll` (`v0.7.0`) into `<FM27 folder>\BepInEx\plugins\FM27Trainer.Bridge\` and restart FM27.

---

<details>
<summary><strong>For developers / Geliştiriciler için</strong></summary>

```bash
# Desktop app (Rust)
cargo build --release -p fm27-trainer
cargo test --workspace

# Optional BepInEx 6 IL2CPP plugin (.NET 8 SDK)
dotnet build bridge/FM27Trainer.Bridge.csproj -c Release -p:FM27Dir="PATH_TO_FM27"
```

| Folder | Contents |
|---|---|
| `app/` | Desktop app: planner, training, history, individual & tactics tabs, OCR |
| `agent/` | Read-only FM27 memory reader + verified training-calendar writer |
| `bridge/` | Optional BepInEx 6 IL2CPP plugin: exact tactical data + intensity/focus events |

Technical details: [docs/TECHNICAL.md](docs/TECHNICAL.md)

</details>

## Credits & License

- Based on [SemHiel2003/fm27-trainer](https://github.com/SemHiel2003/fm27-trainer) by **Sem Hiel** (MIT).
- Thanks to FMSuperScout, FMLibreEditor, fm-lens and fmsave for research. Font: Inter (SIL OFL). Icons: Phosphor (MIT). See [THIRD_PARTY.md](THIRD_PARTY.md).
- License: [MIT](LICENSE)
