use error_chain::error_chain;
use polars_core::prelude::*;
use polars_io::prelude::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn download_covid_data() -> Result<()> {
    let data_url = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv";
    let file_name = "covid_data.csv";
    download_file(data_url, file_name)?;
    Ok(())
}

fn download_file(url: &str, file_name: &str) -> Result<()> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  std::io::Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

fn read_csv(file_name: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(file_name)?
            .has_header(true)
            .finish()
}

fn plot_country(df: &DataFrame, country: &str) -> Result<()> {
    let countries = df.column("country").unwrap().to_owned();

    let columns: Vec<String> = df.get_column_names_owned()
                                 .into_iter()
                                 .skip(4)
                                 .map(|x| x.to_string())
                                 .collect();

    let mask_country: ChunkedArray<BooleanType> = countries.utf8()
                                                      .unwrap()
                                                      .into_iter()
                                                      .map(|x| x.unwrap().starts_with(country))
                                                      .collect();
    let country_data = df.drop_many(&["province","country","Lat","Long"]).filter(&mask_country).unwrap().to_owned();

    let rows: Vec<i64> = country_data.get(0)
                           .unwrap()
                           .into_iter()
                           .map(|x| x.try_extract().unwrap())
                           .collect();

    let mut plot = plotly::Plot::new();
    plot.add_trace(plotly::Scatter::new(columns, rows));
    plot.write_html(format!("{}.html", country));
    Ok(())
}

#[cfg(test)]
use std::collections::HashSet;

#[test]
fn modify_data() {
    let file_name = "covid_data.csv";
    let mut df = read_csv(file_name).unwrap();

    df.rename("Province/State", "province").unwrap();
    df.rename("Country/Region", "country").unwrap();

    let unique_countries = df.column("country").unwrap().to_owned().unique().unwrap();

    let u_countries: HashSet<&str> = unique_countries.utf8().unwrap().into_iter().map(|x| x.unwrap()).filter(|x| x.starts_with("U")).collect();
    assert_eq!(u_countries, HashSet::from(["United Kingdom", "Uruguay", "Uganda", "Uzbekistan", "Ukraine", "United Arab Emirates", "US"]));
}