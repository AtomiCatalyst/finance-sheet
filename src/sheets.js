import {auth} from "./auth.js";
import {google} from "googleapis";

export async function loadSheets(privKey, sheetId) {
    const jwtClient = await auth(privKey);
    const sheets = await google.sheets({version: 'v4', auth: jwtClient});
    return {
        loadRows: async (depositRangeInput) => {
            const {data: {values: rows}} = await sheets
                .spreadsheets
                .values
                .get({
                    spreadsheetId: sheetId,
                    range: depositRangeInput,
                });
            return rows;
        },
        writeRows: async (depositRangeOutput, depositData, deposits) => {
            let totalsData = Object
                .keys(depositData)
                .map(depositType => ({
                    header: depositData[depositType],
                    total: deposits[depositType] || 0
                }))
                .map(headerAndTotal => {
                    let returnObject = {}
                    returnObject[headerAndTotal.header] = headerAndTotal.total;
                    return returnObject;
                });
            const dataToWrite = {
                values: [
                    Object.keys(totalsData),
                    Object.values(totalsData),
                ]
            }
            await sheets
                .spreadsheets
                .values
                .append({
                    spreadsheetId: sheetId,
                    range: depositRangeOutput,
                    resource: dataToWrite,
                    valueInputOption: "RAW",
                }, (err, result) => {
                    if (err) {
                        console.log(err);
                    } else {
                        console.log(
                            '%d cells updated on range: %s',
                            result.data.updates.updatedCells,
                            result.data.updates.updatedRange
                        );
                    }
                });
        }
    }
}