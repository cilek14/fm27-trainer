<p align="center">
  <img src="docs/images/icon.png" width="112" height="112" alt="FM27 Trainer icon">
</p>

<h1 align="center">FM27 Trainer</h1>

<p align="center">
  <strong>Football Manager 27 için hilesiz antrenman yardımcısı.<br>
  A no-cheat training helper for Football Manager 27.</strong>
</p>

<p align="center">
  <img alt="Windows" src="https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows">
  <img alt="FM27" src="https://img.shields.io/badge/Football%20Manager-27-c8102e">
  <img alt="No cheats" src="https://img.shields.io/badge/hile-yok%20%2F%20no%20cheats-3dd68c">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-7c6cff">
</p>

<p align="center">
  <a href="#türkçe">🇹🇷 Türkçe</a> ·
  <a href="#english">🇬🇧 English</a> ·
  <a href="../../releases/latest">İndir / Download</a>
</p>

<p align="center">
  <img src="docs/images/training.png" alt="FM27 Trainer" width="92%">
</p>

---

<a name="türkçe"></a>
## 🇹🇷 Türkçe

### Bu program ne işe yarar?

FM27'de antrenmanı her hafta elle ayarlamak zahmetlidir. FM27 Trainer bu işi senin yerine yapar:

- Takımının **hangi taktik konularda zayıf olduğuna** bakar ve haftalık antrenmanı ona göre hazırlar.
- Yorgun oyuncuları ve yoğun fikstürü fark edip **gerektiğinde dinlenmeye ağırlık verir**.
- **Taktiğe en az uyum sağlayan oyuncuları** listeler.
- Her oyuncu için **sakatlık riskini düşüren antrenman yoğunluğunu** önerir.

### Hile var mı?

**Hayır.** Program yalnızca FM'de senin de elle yapabileceğin şeyleri yapar:

| ✅ Yapar | ❌ Yapmaz |
|---|---|
| Antrenman takvimini doldurur | Oyuncu özelliklerini yükseltmez |
| Oyuncuların antrenman yoğunluğunu seçer | Moral, kondisyon veya form değiştirmez |
| Kadro bilgilerini okur | Para, transfer, sakatlık veya ceza ile oynamaz |

### Ne yapabilirsin?

**📅 Antrenman sekmesi — takım antrenmanı**
- Antrenman, takımının en zayıf olduğu taktik konulara odaklanır.
- Oyun tarzını seçebilirsin: Gegenpress, Tiki-Taka, Kanat Oyunu, Savunma & Kontra ya da Otomatik.
- Özel kamp seçebilirsin: Kondisyon Kampı, Oyuncu Gelişimi veya Derbi/Final Hazırlığı.
- Kendiliğinden yaptıkları:
  - Kadro yorgunsa ya da haftada 2 veya daha fazla maç varsa **daha çok dinlendirir**.
  - Takım taktiği iyice öğrendiğinde **oyuncu gelişimine geçer**.
  - Milli ara ve kış arası haftalarını tanır.
  - FM'nin yardımcı antrenörü programını değiştirirse **senin planını geri koyar**.
- Her haftanın kondisyon, yorgunluk, mutluluk, sakatlık riski ve keskinliğe etkisini gösterir.

**👤 Bireysel sekmesi — oyuncu oyuncu bakış**
- **"Taktiği kim aşağı çekiyor?"** listesi: oyuncular taktiğe uyumu en düşük olandan başlayarak sıralanır. Her oyuncunun asıl sorununu (taktiği anlaması, rolüne uygunluğu, kalitesi) ve ne yapabileceğini görürsün.
- **Akıllı antrenman yoğunluğu:** her oyuncu için *Yarı*, *Normal* veya *Çift* yoğunluk önerilir:
  - Sakat ya da çok yorgun oyuncu → *Yarı* (maçta dinlendir uyarısıyla)
  - 32 yaş ve üstü → en fazla *Normal*
  - Haftada 2 maç varsa → *Çift* yok
  - Dinç genç oyuncu ve rahat bir hafta → *Çift* (daha hızlı gelişim)
- Önerileri tek tıkla hepsine uygulayabilir ya da her oyuncu için ayrı seçebilirsin.

> Program oyuncuları kendi kafasına göre taktikteki rollere **zorlamaz**. Böylece yedek oyuncular kendilerine uymayan bir rolü çalışmak zorunda kalmaz.

### Kurulum

1. **[İndirme sayfasından](../../releases/latest)** `FM27 Trainer.exe` dosyasını indir ve çalıştır. Kurulum gerekmez.
2. FM27'yi aç ve kariyerini yükle. Program oyuna kendiliğinden bağlanır.

**İsteğe bağlı: daha doğru sonuçlar için eklenti**

Oyununda BepInEx kuruluysa `FM27Trainer.Bridge.dll` dosyasını şu klasöre kopyala:

```
<FM27 klasörü>\BepInEx\plugins\FM27Trainer.Bridge\
```

Sonra FM27'yi yeniden başlat. Bu eklentiyle:
- Taktik değerleri ekrandan tahmin edilmek yerine **doğrudan oyundan, kesin olarak** okunur.
- Bireysel sekmesindeki oyuncu listesi dolar. Bunun için oyunda **Taktiksel Verimlilik** ekranını bir kez açman yeterli.
- Antrenman yoğunluğu önerileri tek tıkla oyuna uygulanabilir.

Eklenti yoksa program yine çalışır; taktik değerlerini FM ekranının görüntüsünden okur.

### Dil

Türkçe, İngilizce ve Hollandaca. Bilgisayarının diline göre kendiliğinden seçilir, istersen ayarlardan değiştirebilirsin.

---

<a name="english"></a>
## 🇬🇧 English

### What does it do?

Setting up training by hand every week in FM27 is tedious. FM27 Trainer does it for you:

- It checks **which tactical areas your team is weak in** and builds the weekly schedule around them.
- It notices tired players and busy fixture weeks and **adds rest when needed**.
- It lists **the players who fit your tactic the least**.
- It suggests **an injury-safe training intensity** for each player.

### Is it a cheat?

**No.** It only does things you could also do by hand in FM:

| ✅ It does | ❌ It never does |
|---|---|
| Fill in the training calendar | Boost player attributes |
| Pick each player's training intensity | Change morale, condition or form |
| Read squad information | Touch money, transfers, injuries or bans |

### What can you do with it?

**📅 Training tab: team training**
- Training focuses on your team's weakest tactical areas.
- Pick a playing style: Gegenpress, Possession, Wing Play, Low Block & Counter, or Auto.
- Pick a special camp: Fitness, Player Development, or Derby/Final Prep.
- On its own, it:
  - **Adds rest** when the squad is tired or there are 2 or more matches in a week.
  - **Switches to player development** once the team knows the tactic well.
  - Recognises international and winter breaks.
  - **Puts your plan back** if FM's assistant changes it.
- Shows each week's effect on condition, fatigue, happiness, injury risk and sharpness.

**👤 Individual tab: player by player**
- **"Who drags the tactic down?"**: players sorted from the worst tactical fit up, with each player's main problem (understanding the tactic, suiting the role, quality) and a tip.
- **Smart training intensity:** *Half*, *Normal* or *Double* is suggested for each player:
  - Injured or very tired player → *Half* (with a "rest him" warning)
  - Age 32+ → *Normal* at most
  - 2 matches in a week → no *Double*
  - Fresh young player in a quiet week → *Double* (faster development)
- Apply all suggestions in one click, or choose per player.

> The app does **not** force players into tactic roles, so backups never end up training a role that doesn't suit them.

### Install

1. Download `FM27 Trainer.exe` from the **[download page](../../releases/latest)** and run it. No installation needed.
2. Start FM27 and load your save. The app connects by itself.

**Optional: plugin for more accurate results**

If you have BepInEx installed for FM27, copy `FM27Trainer.Bridge.dll` into:

```
<FM27 folder>\BepInEx\plugins\FM27Trainer.Bridge\
```

Then restart FM27. With the plugin:
- Tactical values are read **exactly, straight from the game**, instead of from a screenshot.
- The player list on the Individual tab fills in. Open the **Tactical Effectiveness** screen in FM once.
- Training intensity suggestions can be applied to the game in one click.

Without the plugin the app still works; it reads tactical values from a screenshot of FM.

### Language

Turkish, English and Dutch, picked automatically from your system language and changeable in Settings.

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
| `app/` | Desktop app: planner, training and individual tabs, screen reading |
| `agent/` | Reads FM27 data (read-only) and writes only the training calendar |
| `bridge/` | Optional BepInEx plugin: exact tactical data and intensity changes |

Technical details: [docs/TECHNICAL.md](docs/TECHNICAL.md)

</details>

## Credits & License

- Based on [SemHiel2003/fm27-trainer](https://github.com/SemHiel2003/fm27-trainer) by **Sem Hiel** (MIT).
- Thanks to FMSuperScout, FMLibreEditor, fm-lens and fmsave for research. Font: Inter (SIL OFL). Icons: Phosphor (MIT). See [THIRD_PARTY.md](THIRD_PARTY.md).
- License: [MIT](LICENSE)
