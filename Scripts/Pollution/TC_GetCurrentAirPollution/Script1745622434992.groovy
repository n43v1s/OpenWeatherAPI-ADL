import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

TestObject requestObj = findTestObject('Object Repository/Pollution/Get_CurrentAirPollution')

def response = WS.sendRequest(requestObj)
WS.verifyResponseStatusCode(response, 200)
WS.verifyResponseStatusCodeInRange(response, 200, 299)

def jsonResponse = new JsonSlurper().parseText(response.getResponseText())

// Verify Location
double geoLat = Double.parseDouble(GlobalVariable.VALID_LATITUDE.trim())
double geoLon = Double.parseDouble(GlobalVariable.VALID_LONGITUDE.trim())
double forecastLat = jsonResponse.coord.lat
double forecastLon = jsonResponse.coord.lon
def tolerance = 0.001
assert Math.abs(geoLat - forecastLat) <= tolerance : "Latitude doesn't match"
assert Math.abs(geoLon - forecastLon) <= tolerance : "Longitude doesn't match"